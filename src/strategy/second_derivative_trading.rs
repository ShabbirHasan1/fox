/*
 * This strategy does not work well at the moment, so don't use it. Also, a lot of the buy/sell
 * indicators are hardcoded for the ETH market.
*/


use rand::{Rng, thread_rng};
use tokio_tungstenite::connect_async;
use futures_util::{StreamExt, SinkExt};
use chrono::{DateTime, Utc};
use colored::Colorize;

use std::sync::Arc;

use crate::dydx::{InternalAccount, Markets, Position, Side, OrderType, TradeData};

use crate::analysis::Ring;

pub struct SecondDerivative;

#[derive(Debug, Clone)]
pub struct OpenPosition {
    open_price: f64,
    id: String,
}

impl SecondDerivative {

    pub(crate) async fn run(account: InternalAccount, market: Markets, testnet: bool) -> anyhow::Result<()> {

        let mut open_long_positions: Vec<OpenPosition> = Vec::<OpenPosition>::new();
        let mut open_short_positions: Vec<OpenPosition> = Vec::<OpenPosition>::new();

        let now = Utc::now().timestamp() as u64;
        let position_id = account.position_id();
        let (tx, mut rx) = tokio::sync::mpsc::channel::<(f64, u64)>(25);

        // <------------------------------------------------------------->
        // This thread just manages the data feed from DYDX
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
            });
            read_future.await;
            Ok::<(), anyhow::Error>(())
        });

        // <--------------------------------------------------->
        //
        // This theoretically helps us make good decisions while 
        // market making
        //
        //<---------------------------------------------------->
        let mut large_ring: Ring<100> = Ring::initialize(100);
        let mut small_ring: Ring<10> = Ring::initialize(10);
        // <--------------------------------------------------->
        //
        // The size of the ring is proportional to the activity 
        // of the bot: the smaller the ring, the more active the 
        // bot will be.
        //
        //<---------------------------------------------------->



        let mut onetime_flag = false;
        let account_clone = Arc::new(account);
        loop {

            // <---------------------------->
            //      Check the data feed
            // <---------------------------->
            let mut recv = rx.try_recv();
            while recv.is_ok() {
                let (price, timestamp) = recv.unwrap();
                large_ring.update(price, timestamp);
                small_ring.update(price, timestamp);
                recv = rx.try_recv();
            }
                
            let most_recent_price = small_ring.most_recent_price();

            if large_ring.full() {
                
                if !onetime_flag { 
                    println!("{} {}", "[+]".yellow().bold(), "The bot has collected enough data and will now execute trades.".yellow());
                    onetime_flag = true;
                }

                let sr_first_deriv = small_ring.avg_price_change();
                let lr_second_derivative = large_ring.estimate_avg_second_derivative();

                // <-------------------------------------------------->
                //  This is the sell indicator: In order for the bot
                //  to open a short position...
                //  
                //  1. The rolling average of the smaller sliding window
                //     must be less than the rolling average of the 
                //     larger sliding window (post-death cross)
                //  2. The asset's average price per second must be 
                //     less than -0.2. That is, within the timeframe of
                //     the smaller sliding window, the asset is going down
                //     in value at a rate greater than $0.20 per second.
                //  3. The convexity of the larger sliding window is 
                //     negative. This is equivalent to saying that the 
                //     second derivative of the rolling average is negative.
                //
                // <--------------------------------------------------->

                if (small_ring.average() < large_ring.average()) && sr_first_deriv <= -0.2f64 && lr_second_derivative < 0f64 {

                    let thread_account = Arc::clone(&account_clone);
                    let market_account = Arc::clone(&thread_account);
                    let trailing_stop_account = Arc::clone(&thread_account);

                    let random_id = thread_rng().gen::<u128>();
                    let trailing_stop_side = Side::BUY;
                    let market_side = Side::SELL;
                    let price = most_recent_price - 1f64;
                   
                    let account_value = thread_account.equity(testnet).await?;
                    let order_size = format!("{:.3}", (sr_first_deriv * -1f64) * (account_value/price));
                    
                    let order_size = "0.01".to_string();
                    let market_position = Position::new(price.to_string(), order_size.clone(), market_side);
                    let stop_position = Position::new(format!("{:.1}", price + (price * 0.03)), order_size.clone(), trailing_stop_side);
                    let trade_data = TradeData::new(None, None, Some(false));

                    let order_response: serde_json::Value = tokio::spawn(async move {
                        let response = market_account.open_order(
                            market, market_position, 
                            OrderType::MARKET, 
                            "0.02".to_string(), 
                            format!("{}", random_id), 
                            position_id, 
                            false,
                            trade_data,
                            testnet
                        ).await.unwrap();

                        serde_json::from_str(&response).unwrap()
                    
                    }).await.unwrap();

                    let trade_data = TradeData::new(Some("0.1".to_string()), None, Some(false));
                    let trail_id: serde_json::Value = tokio::spawn(async move {
                        let trail_id = trailing_stop_account.open_order(
                            market, stop_position, 
                            OrderType::TRAILING_STOP, 
                            "0.02".to_string(), 
                            format!("{}", random_id + 1), 
                            position_id, 
                            false, 
                            trade_data,
                            testnet
                        ).await.unwrap();

                        serde_json::from_str(&trail_id).unwrap()
                    }).await.unwrap();

                    let trail_id = trail_id["cancelOrder"]["id"].as_str().unwrap_or("0");

                    let market_price = order_response["order"]["price"].as_str();
                    if market_price.is_some() { open_short_positions.push(OpenPosition::new(market_price.unwrap().parse::<f64>().unwrap(), trail_id.to_string())); }

                    println!("{} {} {} {} {}.", "[+]".green().bold(), "Opened sell order of size".green(), order_size.to_string().green(), "at price".green(), price.to_string().green());

                }



                // <-------------------------------------------------->
                //  This is the buy indicator: it is largely the 
                //  inverse of the short indicator. In order to open
                //  a long position, the following must be true:
                //  
                //  1. The rolling average of the smaller sliding window
                //     must be greate than the rolling average of the 
                //     larger sliding window (post-golden cross).
                //  2. The asset's average price per second must be 
                //     greater than 0.2. That is, within the timeframe of
                //     the smaller sliding window, the asset is going up
                //     in value at a rate greater than $0.20 per second.
                //  3. The convexity of the smaller sliding window is 
                //     positive. This is equivalent to saying that the 
                //     second derivative of the rolling average is positive.
                //
                // <--------------------------------------------------->

                if sr_first_deriv >= 0.2f64 && (small_ring.average() > large_ring.average()) && lr_second_derivative > 0f64 {

                    let thread_account = Arc::clone(&account_clone);
                    let market_account = Arc::clone(&thread_account);
                    let trailing_stop_account = Arc::clone(&thread_account);

                    let random_id = thread_rng().gen::<u128>();
                    let trailing_stop_side = Side::SELL;
                    let market_side = Side::BUY;
                    let price = most_recent_price + 1f64; // Add a buck to make sure the market order executes.

                    let account_value = thread_account.equity(testnet).await?;
                    let order_size = format!("{:.3}", sr_first_deriv * (account_value/price));
                    
                    let order_size = "0.01".to_string();
                    let market_position = Position::new(price.to_string(), order_size.clone(), market_side);
                    let stop_position = Position::new(format!("{:.1}", price - (price * 0.03)), order_size.clone(), trailing_stop_side);
                    let trade_data = TradeData::new(None, None, Some(false));

                    let order_response: serde_json::Value = tokio::spawn(async move {
                        let response = market_account.open_order(
                            market, market_position, 
                            OrderType::MARKET, 
                            "0.02".to_string(), 
                            format!("{}", random_id), 
                            position_id, 
                            false, 
                            trade_data,
                            testnet
                        ).await.unwrap();

                        serde_json::from_str(&response).unwrap()

                    }).await.unwrap();
                    
                    let trade_data = TradeData::new(Some("-0.1".to_string()), None, Some(false));

                    let trail_id: serde_json::Value = tokio::spawn(async move {
                        let trail_id = trailing_stop_account.open_order(
                            market, stop_position, 
                            OrderType::TRAILING_STOP, 
                            "0.02".to_string(), 
                            format!("{}", random_id + 1), 
                            position_id, 
                            false, 
                            trade_data,
                            testnet
                        ).await.unwrap();

                        serde_json::from_str(&trail_id).unwrap()
                    }).await.unwrap();

                    let trail_id = trail_id["cancelOrder"]["id"].as_str().unwrap_or("0");

                    let market_price = order_response["order"]["price"].as_str();
                    if market_price.is_some() { open_long_positions.push(OpenPosition::new(market_price.unwrap().parse::<f64>().unwrap(), trail_id.to_string())); }

                    println!("{} {} {} {} {}.", "[+]".green().bold(), "Opened buy order of size".green(), order_size.to_string().green(), "at price".green(), price.to_string().green());

                }
            }

            // <-------------------------------------->
            //  This is where I try to unload my open
            //  positions back into the market. 
            //  The vectors which hold the open positions
            //  should be pretty short so we just iterate
            //  thorugh them and sell the ones we can sell.
            //  
            //  It is possible that a position we buy can not
            //  be sold for a profit, which is also why we have 
            //  the trailing stops. Essentially, the bot bets
            //  that the strategy will net more $$$ than the amount
            //  it will lose from the small positions which are 
            //  covered by the trailing stops.
            //  <-------------------------------------> 

            let mut new_longs = Vec::<OpenPosition>::new();
            for position in open_long_positions.iter() {
                if position.open_price < most_recent_price - 1f64 {

                    let thread_account = Arc::clone(&account_clone);
                    let market_account = Arc::clone(&thread_account);

                    let random_id = thread_rng().gen::<u128>();
                    let market_side = Side::SELL;

                    let price = most_recent_price - 1f64;
                    let order_size = "0.01".to_string();
                    let market_position = Position::new(price.to_string(), order_size.clone(), market_side);
                    let cancel_id = position.id.clone();

                    let cancel_check: serde_json::Value = tokio::spawn(async move {
                        let cancel_check = thread_account.cancel_order(cancel_id, testnet).await.unwrap();
                        serde_json::from_str(&cancel_check).unwrap()
                    }).await.unwrap();
                    
                    let cancel_check = cancel_check["cancelOrder"]["id"].as_str();
                    let trade_data = TradeData::new(None, None, Some(false));

                    if cancel_check.is_some() {
                        let did_close: serde_json::Value = tokio::spawn(async move {
                            let close_position = market_account.open_order(
                                market, market_position, 
                                OrderType::MARKET, 
                                "0.02".to_string(), 
                                format!("{}", random_id), 
                                position_id, 
                                false, 
                                trade_data,
                                testnet
                            ).await.unwrap();

                            serde_json::from_str(&close_position).unwrap()
                        }).await.unwrap();
                        let market_sale = did_close["order"]["price"].as_str();
                        if market_sale.is_some() { println!("{} {} {}", "[+]".green().bold(), "Long position closed for profit at price".green(), market_sale.unwrap().green()); }
                    }
                }
                else { new_longs.push(position.clone()); }
            }
            open_long_positions = new_longs;

            let mut new_shorts = Vec::<OpenPosition>::new();
            for position in open_short_positions.iter() {
                if position.open_price > most_recent_price + 1f64 {

                    let thread_account = Arc::clone(&account_clone);
                    let market_account = Arc::clone(&thread_account);

                    let random_id = thread_rng().gen::<u128>();
                    let market_side = Side::BUY;

                    let price = most_recent_price + 1f64;
                    let order_size = "0.01".to_string();
                    let market_position = Position::new(price.to_string(), order_size.clone(), market_side);
                    let cancel_id = position.id.clone();

                    let cancel_check: serde_json::Value = tokio::spawn(async move {
                        let cancel_check = thread_account.cancel_order(cancel_id, testnet).await.unwrap();
                        serde_json::from_str(&cancel_check).unwrap()
                    }).await.unwrap();
                    let cancel_check = cancel_check["cancelOrder"]["id"].as_str();
                    let trade_data = TradeData::new(None, None, Some(false));

                    if cancel_check.is_some() {
                        let did_close: serde_json::Value = tokio::spawn(async move {
                            let did_close = market_account.open_order(
                                market, 
                                market_position, 
                                OrderType::MARKET, 
                                "0.02".to_string(), 
                                format!("{}", random_id), 
                                position_id, 
                                false, 
                                trade_data,
                                testnet
                            ).await.unwrap();
                            serde_json::from_str(&did_close).unwrap()
                        }).await.unwrap();
            
                        let market_sale = did_close["order"]["price"].as_str();
                        if market_sale.is_some() { println!("{} {} {}", "[+]".green().bold(), "Short position closed for profit at price".green(), market_sale.unwrap().green()); }
                    }
                }
                else { new_shorts.push(position.clone()); }
            }
            open_short_positions = new_shorts;
        }
    }
}


impl OpenPosition {

    fn new(open_price: f64, id: String) -> Self {
        Self {
            open_price,
            id,
        }
    }
}
