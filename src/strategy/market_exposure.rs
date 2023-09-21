use rand::{Rng, thread_rng};
use tokio_tungstenite::connect_async;
use futures_util::{StreamExt, SinkExt};
use chrono::{DateTime, Utc};
use colored::Colorize;
use tokio::time;
use std::{
    sync::Arc,
};

use crate::dydx::{InternalAccount, Markets, Position, Side, OrderType, Exposure, TradeData};

use crate::analysis::Partition;

pub struct MarketExposure;

#[derive(Debug, Clone)]
struct ActiveTrade {
    value: f64,
    epoch: usize,
    side: Side,
}

struct SendData {
    market: Markets,
    tx: tokio::sync::mpsc::Sender<f64>,
}

#[derive(Debug)]
struct ReceiveData<const N: usize> {
    market: Markets,
    partitions: [Partition; N],
    trade: Vec<ActiveTrade>,
    rx: tokio::sync::mpsc::Receiver<f64>,
    most_recent_price: f64,
}

const TIMESTEPS: usize = 6;
const WINDOW: usize = 3600; // one hour windows
const SELL_THRESHOLD: f64 = 0.1; // profit to automatically take

macro_rules! market_exposure {
    (($acc:ident, $testnet:ident, $position_id:ident); $($mark:ident),+ $(,)?) => {{
        let interval = tokio::time::Duration::new((WINDOW/TIMESTEPS) as u64, 0);
        let mut checkpoint = tokio::time::Instant::now() + interval;
        let mut epoch = 0usize;
        let mut index = 0usize;
        let arc_account = Arc::new($acc);
        loop {
            tokio::select! {
                $(
                    price = $mark.rx.recv() => {
                        if let Some(value) = price {
                            // println!("{} {} {} {} {}", "[i]".purple().bold(), "Latest".purple(), format!("{}", $mark.market).purple(), "is".purple(), value.to_string().purple());
                            $mark.partitions[index].update(value);
                            $mark.most_recent_price = value;
                        }
                    }
                )+
                _ = tokio::time::sleep_until(checkpoint) => {
                    checkpoint = tokio::time::Instant::now() + interval;
                    epoch += 1usize;
                    index = epoch % TIMESTEPS;
                    if epoch >= TIMESTEPS {
                    // Execute necessary trades
                        
                    // Check if concave up (buy)
                        $(
                            if $mark.dips(epoch, $mark.market.dip_delta(), $mark.market.price_delta()) {
                                let random_id = thread_rng().gen::<u128>();
                                let order_size = $mark.market.default_order_size();
                                let market_position = Position::new(format!("{:.3}", $mark.most_recent_price + 1f64), order_size.to_string(), Side::BUY);
                                let internal_account = Arc::clone(&arc_account);
                                let trade_data = TradeData::new(None, None, Some(false));
                                let tokio_response: serde_json::Value = tokio::spawn(async move {
                                    let response = internal_account.open_order(
                                        $mark.market,
                                        market_position,
                                        OrderType::MARKET,
                                        "0.01".to_string(),
                                        random_id.to_string(),
                                        $position_id,
                                        false,
                                        trade_data,
                                        $testnet
                                    ).await.unwrap();
                                    serde_json::from_str(&response).unwrap()
                                }).await.unwrap();
                                let market_sale = tokio_response["order"]["price"].as_str();
                                if market_sale.is_some() {
                                    let value = order_size as f64 * market_sale.unwrap().parse::<f64>().unwrap();
                                    println!(
                                        "{} {} {} {} {}",
                                        "[+]".green().bold(),
                                        format!("{}", $mark.market).green(),
                                        "position bought for a total of".green(),
                                        value.to_string().green(),
                                        "USD".green()
                                    );
                                    if !$mark.has_active_trade() { $mark.trade = vec![ActiveTrade::new(value, Side::BUY, epoch)]; }
                                    else { $mark.trade.push(ActiveTrade::new(value, Side::BUY, epoch)); }
                                }
                            }

                        // Check if concave down (sell)
                            if $mark.soars(epoch, $mark.market.dip_delta(), $mark.market.price_delta()) {
                                let random_id = thread_rng().gen::<u128>();
                                let order_size = $mark.market.default_order_size();
                                let market_position = Position::new(format!("{:.3}", $mark.most_recent_price + 1f64), order_size.to_string(), Side::SELL);
                                let internal_account = Arc::clone(&arc_account);
                                let trade_data = TradeData::new(None, None, Some(false));
                                let tokio_response: serde_json::Value = tokio::spawn(async move {
                                    let response = internal_account.open_order(
                                        $mark.market,
                                        market_position,
                                        OrderType::MARKET,
                                        "0.01".to_string(),
                                        random_id.to_string(),
                                        $position_id,
                                        false,
                                        trade_data,
                                        $testnet
                                    ).await.unwrap();
                                    serde_json::from_str(&response).unwrap()
                                }).await.unwrap();
                                let market_sale = tokio_response["order"]["price"].as_str();
                                if market_sale.is_some() {
                                    let value = order_size as f64 * market_sale.unwrap().parse::<f64>().unwrap();
                                    println!(
                                        "{} {} {} {} {}",
                                        "[+]".red().bold(),
                                        format!("{}", $mark.market).red(),
                                        "position bought for a total of".red(),
                                        value.to_string().red(),
                                        "USD".red()
                                    );
                                    if !$mark.has_active_trade() { $mark.trade = vec![ActiveTrade::new(value, Side::SELL, epoch)]; }
                                    else { $mark.trade.push(ActiveTrade::new(value, Side::SELL, epoch)); }
                                }
                            }

                        // Close positions if possible
                            if !$mark.trade.is_empty() {
                                let mut new_trades = Vec::new();
                                let size = $mark.market.default_order_size();
                                for i in 0..$mark.trade.len() {
                                    let trade = $mark.trade.remove(i);
                                    if (($mark.most_recent_price * size as f64) - trade.value >= SELL_THRESHOLD) || (epoch - trade.epoch >= TIMESTEPS) {
                                        let random_id = thread_rng().gen::<u128>();
                                        let market_position = Position::new(format!("{:.3}", $mark.most_recent_price + 1f64), size.to_string(), trade.side.other());
                                        let account = Arc::clone(&arc_account);
                                        let trade_data = TradeData::new(None, None, Some(false));
                                        let tokio_response: serde_json::Value = tokio::spawn(async move {
                                            let response = account.open_order(
                                                $mark.market,
                                                market_position,
                                                OrderType::MARKET,
                                                "0.01".to_string(),
                                                random_id.to_string(),
                                                $position_id,
                                                false,
                                                trade_data,
                                                $testnet
                                            ).await.unwrap();
                                            serde_json::from_str(&response).unwrap()
                                        }).await.unwrap();
                                        let market_sale = tokio_response["order"]["price"].as_str();
                                        if market_sale.is_some() {
                                            let value = size as f64 * market_sale.unwrap().parse::<f64>().unwrap();
                                            println!(
                                                "{} {} {} {} {}",
                                                "[+]".yellow().bold(),
                                                format!("{}", $mark.market).yellow(),
                                                "position closed for a total of".yellow(),
                                                value.to_string().yellow(),
                                                "USD".yellow()
                                            );
                                        }
                                    }
                                    else {
                                        new_trades.push(trade);
                                    }
                                }
                                $mark.trade = new_trades;
                            }

                            $mark.partitions[index].wipe();
                        )+
                    }
                    println!(
                        "{} {} {} {}",
                        "[+]".blue().bold(),
                        "Epoch".blue(),
                        epoch.to_string().blue(),
                        "completed.".blue(),
                    );
                }
            }
        }
    }};
}

impl MarketExposure {

    pub(crate) async fn run(account: InternalAccount, exposure: Exposure, testnet: bool) -> anyhow::Result<()> {

        let now = Utc::now().timestamp() as u64;
        let position_id = account.position_id();


        // <------------------------------------------------------------->
        //      This thread just manages the data feed from DYDX
        // <------------------------------------------------------------->
        
        let mut send_data = Vec::new();
        let mut receive_data: Vec<ReceiveData<TIMESTEPS>> = Vec::new();
        let market_vector = Markets::vector();
        for market in market_vector {
            let (tx, rx) = tokio::sync::mpsc::channel::<f64>(25);
            send_data.push(SendData::new(market, tx));
            receive_data.push(ReceiveData::new(market, rx));
        }

        for sdata in send_data {
        
            tokio::spawn(async move {

                let url = {
                    if testnet { url::Url::parse("wss://api.stage.dydx.exchange/v3/ws").unwrap() }
                    else { url::Url::parse("wss://api.dydx.exchange/v3/ws").unwrap() }
                };

                let (socket, _response) = connect_async(url).await?;
                let (mut write, read) = socket.split();

                let message = sdata.market.trade_feed_message();
                write.send(message).await?;

                let read_future = read.for_each(|message| async {
                    if let Err(e) = message { println!("{} {} {:?}", "[-]".red().bold(), "Failed to read message with error:".red(), e); }
                    else {
                        let data: Result<serde_json::Value, serde_json::Error> = serde_json::from_slice(message.expect("Failed to unwrap message").into_data().as_slice());
                        if let Ok(d) = data {
                            let arr = &d["contents"]["trades"];
                            if arr.is_array() {
                                for object in arr.as_array().expect("ERR2") {
                                    let obj_price = object["price"].as_str().expect("Failed to index price").parse::<f64>().expect("Failed to parse price as a float");
                                    let obj_time = DateTime::parse_from_rfc3339(object["createdAt"].as_str().expect("Cannot find timestamp")).expect("Failed to parse as rfc3339").timestamp() as u64;
                                    if obj_time > now { 
                                        sdata.tx.clone().send(obj_price).await.expect("Should not fail here"); 
                                    } // We do not want the old trades;
                                }
                            }
                        }
                    }
                });
                read_future.await;
                Ok::<(), anyhow::Error>(())
            }); 
        }

        let mut xlm = receive_data.pop().unwrap();
        let mut zec = receive_data.pop().unwrap();
        let mut ada = receive_data.pop().unwrap();
        let mut rune = receive_data.pop().unwrap();
        let mut yfi = receive_data.pop().unwrap();
        let mut uma = receive_data.pop().unwrap();
        let mut mkr = receive_data.pop().unwrap();
        let mut xmr = receive_data.pop().unwrap();
        let mut ltc = receive_data.pop().unwrap();
        let mut bch = receive_data.pop().unwrap();
        let mut etc = receive_data.pop().unwrap();
        let mut uni = receive_data.pop().unwrap();
        let mut celo = receive_data.pop().unwrap();
        let mut sushi = receive_data.pop().unwrap();
        let mut inch = receive_data.pop().unwrap();
        let mut xtz = receive_data.pop().unwrap();
        let mut trx = receive_data.pop().unwrap();
        let mut icp = receive_data.pop().unwrap();
        let mut eos = receive_data.pop().unwrap();
        let mut zrx = receive_data.pop().unwrap();
        let mut algo = receive_data.pop().unwrap();
        let mut comp = receive_data.pop().unwrap();
        let mut aave = receive_data.pop().unwrap();
        let mut enj = receive_data.pop().unwrap();
        let mut atom = receive_data.pop().unwrap();
        let mut doge = receive_data.pop().unwrap();
        let mut dot = receive_data.pop().unwrap();
        let mut snx = receive_data.pop().unwrap();
        let mut link = receive_data.pop().unwrap();
        let mut crv = receive_data.pop().unwrap();
        let mut near = receive_data.pop().unwrap();
        let mut matic = receive_data.pop().unwrap();
        let mut avax = receive_data.pop().unwrap();
        let mut fil = receive_data.pop().unwrap();
        let mut sol = receive_data.pop().unwrap();
        let mut eth = receive_data.pop().unwrap();
        let mut btc = receive_data.pop().unwrap();

        // There may be a more succinct way to do this if we use unsafe code (specifically utilizing
        // *mut). There would be safety since we never mutate the same index in the vector at the
        // same time, but I chose not to use any unsafe code.
        market_exposure!(
            (account, testnet, position_id);
            xlm,
            zec,
            ada,
            rune,
            yfi,
            uma,
            mkr,
            xmr,
            ltc,
            bch,
            etc,
            uni,
            celo,
            sushi,
            inch,
            xtz,
            trx,
            icp,
            eos,
            zrx,
            algo,
            comp,
            aave,
            enj,
            atom,
            doge,
            dot,
            snx,
            link,
            crv,
            near,
            matic,
            avax,
            fil,
            sol,
            eth,
            btc,
        );
    }
}
       
impl<const N: usize> ReceiveData<N> {
    
    fn new(market: Markets, rx: tokio::sync::mpsc::Receiver<f64>) -> Self {
        Self {
            market,
            partitions: [Partition::new(); N],
            trade: Vec::new(),
            rx,
            most_recent_price: 0f64,
        }
    }

    fn has_active_trade(&self) -> bool {
        !self.trade.is_empty()
    }

    // Will not panic because function is only called when index >= N.
    fn dips(&self, index: usize, dip_delta: f64, price_delta: f64) -> bool {
        let most_recent_average = self.partitions[(index - 1usize) % N].average();
        let oldest_average = self.partitions[(index - N) % N].average();
        let potential_drop = self.partitions[(index - (N-1)) % N].average();
        if (most_recent_average - oldest_average).abs() < price_delta && (oldest_average - potential_drop) > dip_delta {
            return true;
        }
        false
    }

    fn soars(&self, index: usize, soar_delta: f64, price_delta: f64) -> bool {
        let most_recent_average = self.partitions[(index - 1usize) % N].average();
        let oldest_average = self.partitions[(index - N) % N].average();
        let potential_soar = self.partitions[(index - (N-1)) % N].average();
        if (most_recent_average - oldest_average).abs() < price_delta && (potential_soar - oldest_average) > soar_delta {
            return true;
        }
        false
    }
}

impl SendData {
    
    fn new(market: Markets, tx: tokio::sync::mpsc::Sender<f64>) -> Self {
        Self {
            market,
            tx,
        }
    }
}

impl ActiveTrade {

    fn new(value: f64, side: Side, epoch: usize) -> Self {
        Self {
            value,
            epoch,
            side,
        }
    }
}
