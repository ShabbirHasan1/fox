use rand::{Rng, thread_rng};
use tokio_tungstenite::connect_async;
use futures_util::{StreamExt, SinkExt};
use chrono::{DateTime, Utc};
use colored::Colorize;

use std::sync::Arc;

use crate::dydx::{InternalAccount, Markets, Position, Side, OrderType, Exposure, TradeData};

use crate::analysis::Ring;

pub struct MarketMake;

impl MarketMake {

    pub(crate) async fn run(account: InternalAccount, market: Markets, exposure: Exposure, testnet: bool) -> anyhow::Result<()> {

        let now = Utc::now().timestamp() as u64;
        let position_id = account.position_id();
        let (tx, mut rx) = tokio::sync::mpsc::channel::<(f64, u64)>(25);

        // <------------------------------------------------------------->
        //      This thread just manages the data feed from DYDX
        // <------------------------------------------------------------->
        tokio::spawn(async move {

            let url = {
                if testnet { url::Url::parse("wss://api.stage.dydx.exchange/v3/ws").unwrap() }
                else { url::Url::parse("wss://api.dydx.exchange/v3/ws").unwrap() }
            };

            let (socket, _response) = connect_async(url).await?;
            let (mut write, read) = socket.split();

            let message = market.trade_feed_message();
            write.send(message).await?;

            let read_future = read.for_each(|message| async {
                if let Err(e) = message { println!("{} {} {:?}", "[-]".red().bold(), "Failed to read message with error:".red(), e); }
                else {
                    let data: Result<serde_json::Value, serde_json::Error> = serde_json::from_slice(message.unwrap().into_data().as_slice());
                    if let Ok(d) = data {
                        let arr = &d["contents"]["trades"];
                        if arr.is_array() {
                            for object in arr.as_array().unwrap() {
                                let obj_price = object["price"].as_str().unwrap().parse::<f64>().unwrap();
                                let obj_time = DateTime::parse_from_rfc3339(object["createdAt"].as_str().unwrap()).unwrap().timestamp() as u64;
                                if obj_time > now { tx.clone().send((obj_price, obj_time)).await.unwrap(); } // We do not want the old trades;
                            }
                        }
                    }
                }
            });
            read_future.await;
            Ok::<(), anyhow::Error>(())
        }); 

        let mut sliding_window = Ring::<200>::initialize(200);
        let account_clone = Arc::new(account);
        let mut onetime_flag = true;
        loop {
            let buy_clone = Arc::clone(&account_clone);
            let sell_clone = Arc::clone(&buy_clone);

            // <---------------------------->
            //      Check the data feed
            // <---------------------------->
            let mut recv = rx.try_recv();
            while recv.is_ok() {
                let (price, timestamp) = recv.unwrap();
                sliding_window.update(price, timestamp);
                recv = rx.try_recv();
            }
            // <--------------------------------------->
            //      When there is low(ish) market 
            //      volatility the bot will open 
            //      long/short positions at a spread 
            //      depending on the type of exposure
            // <--------------------------------------->
            if sliding_window.full() {
                if onetime_flag { println!("{} {}", "[+]".green().bold(), "The bot has gathered enough information and will now begin trading".green()); onetime_flag = false; }
                  // if sliding_window.avg_price_change() < market.exposure(exposure) {
                  if (sliding_window.average() - sliding_window.most_recent_price()).abs() < 0.1 {
                    let target_liquidity_price = sliding_window.most_recent_price();
                    let buy_position = Position::new(format!("{:.1}", target_liquidity_price - 0.12), "0.01".to_string(), Side::BUY); // TODO: Do not hardcode the target spread.
                    let sell_position = Position::new(format!("{:.1}", target_liquidity_price + 0.12), "0.01".to_string(), Side::SELL);
                    let random_id = rand::thread_rng().gen::<u128>();
                    let trade_data1 = TradeData::new(None, None, None);
                    let trade_data2 = TradeData::new(None, None, None);
                    let buy_position_response: Result<serde_json::Value, anyhow::Error> = tokio::spawn(async move {
                        let buy_response = buy_clone.open_order(
                            Markets::ETH,
                            buy_position,
                            OrderType::LIMIT,
                            "0.02".to_string(),
                            format!("{}", random_id),
                            position_id,
                            true,
                            trade_data1,
                            testnet
                        ).await?;
                        Ok(serde_json::from_str(&buy_response).unwrap())
                    }).await.unwrap();
                    let sell_position_response: Result<serde_json::Value, anyhow::Error> = tokio::spawn(async move {
                        let sell_response = sell_clone.open_order(
                            Markets::ETH,
                            sell_position,
                            OrderType::LIMIT,
                            "0.02".to_string(),
                            format!("{}", random_id + 1),
                            position_id,
                            true,
                            trade_data2,
                            testnet
                        ).await?;
                        Ok(serde_json::from_str(&sell_response).unwrap())
                    }).await.unwrap();
                    if sell_position_response.is_ok() {
                        let sell_position_response = sell_position_response.unwrap();
                        if sell_position_response["order"]["price"].as_str().is_some() {
                            println!("{} {} {}", "[+]".green().bold(), "Placed sell limit order at".green(), sell_position_response["order"]["price"].as_str().unwrap().green());
                        }
                        else {
                            println!("{} {} {:?}", "[-]".red().bold(), "Error opening sell limit order. Reason:".red(), sell_position_response);
                        }
                    }
                    if buy_position_response.is_ok() {
                        let buy_position_response = buy_position_response.unwrap();
                        if buy_position_response["order"]["price"].as_str().is_some() {
                            println!("{} {} {}", "[+]".green().bold(), "Placed buy limit order at".green(), buy_position_response["order"]["price"].as_str().unwrap().green());
                        }
                        else {
                            println!("{} {} {:?}", "[-]".red().bold(), "Error opening buy limit order. Reason:".red(), buy_position_response);
                        }
                    }
                }
            }
        }
    }
}

