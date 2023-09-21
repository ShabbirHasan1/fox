use rand::{Rng, thread_rng};
use tokio_tungstenite::connect_async;
use futures_util::{StreamExt, SinkExt};
use chrono::{DateTime, Utc};
use colored::Colorize;

use std::sync::Arc;

use crate::dydx::{InternalAccount, Markets, Position, Side, OrderType, Exposure, TradeData};

use crate::analysis::Ring;

pub struct StructuredEntropy;

impl StructuredEntropy {

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

        loop {
            let temp = rx.try_recv();
        }
    }
}

