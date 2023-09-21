/*
 * Utility file for routing perpetuals to their associated websocket address
*/

use tokio_tungstenite::tungstenite::protocol::Message;
use primitive_types::U512;

use crate::dydx::{Markets, Exposure, Side};
use crate::dydx::constants::*;

use std::fmt;

impl Side {

    pub fn other(&self) -> Side {
        if *self == Side::BUY { return Side::SELL; }
        Side::BUY
    }
}

impl Markets {

    pub fn orderbook_feed_message(&self) -> Message {
        
        match *self {
            Markets::BTC => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "BTC-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::ETH => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "ETH-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::SOL => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "SOL-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::FIL => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "FIL-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::AVAX => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "AVAX-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::MATIC => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "MATIC-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::NEAR => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "NEAR-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::CRV => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "CRV-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::LINK => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "LINK-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::SNX => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "SNX-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::DOT => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "DOT-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::DOGE => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "DOGE-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::ATOM => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "ATOM-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::ENJ => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "ENJ-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::AAVE => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "AAVE-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::COMP => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "COMP-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::ALGO => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "ALGO-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::ZRX => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "ZRX-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::EOS => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "EOS-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::ICP => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "ICP-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::TRX => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "TRX-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::XTZ => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "XTZ-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::INCH => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "1INCH-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::SUSHI => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "SUSHI-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::CELO => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "CELO-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::UNI => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "UNI-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::ETC => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "ETC-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::BCH => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "BCH-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::LTC => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "LTC-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::XMR => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "XMR-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::MKR => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "MKR-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::UMA => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "UMA-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::YFI => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "YFI-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::RUNE => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "RUNE-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::ADA => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "ADA-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::ZEC => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "ZEC-USD", "includeOffsets": true }"#.to_string()+"\n"),
            Markets::XLM => Message::Text(r#"{ "type": "subscribe", "channel": "v3_orderbook", "id": "XLM-USD", "includeOffsets": true }"#.to_string()+"\n"),
        }
    }

    pub fn trade_feed_message(&self) -> Message {

        match *self {
            Markets::BTC => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "BTC-USD" }"#.to_string()+"\n"),
            Markets::ETH => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "ETH-USD" }"#.to_string()+"\n"),
            Markets::SOL => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "SOL-USD" }"#.to_string()+"\n"),
            Markets::FIL => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "FIL-USD" }"#.to_string()+"\n"),
            Markets::AVAX => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "AVAX-USD" }"#.to_string()+"\n"),
            Markets::MATIC => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "MATIC-USD" }"#.to_string()+"\n"),
            Markets::NEAR => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "NEAR-USD" }"#.to_string()+"\n"),
            Markets::CRV => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "CRV-USD" }"#.to_string()+"\n"),
            Markets::LINK => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "LINK-USD" }"#.to_string()+"\n"),
            Markets::SNX => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "SNX-USD" }"#.to_string()+"\n"),
            Markets::DOT => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "DOT-USD" }"#.to_string()+"\n"),
            Markets::DOGE => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "DOGE-USD" }"#.to_string()+"\n"),
            Markets::ATOM => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "ATOM-USD" }"#.to_string()+"\n"),
            Markets::ENJ => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "ENJ-USD" }"#.to_string()+"\n"),
            Markets::AAVE => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "AAVE-USD" }"#.to_string()+"\n"),
            Markets::COMP => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "COMP-USD" }"#.to_string()+"\n"),
            Markets::ALGO => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "ALGO-USD" }"#.to_string()+"\n"),
            Markets::ZRX => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "ZRX-USD" }"#.to_string()+"\n"),
            Markets::EOS => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "EOS-USD" }"#.to_string()+"\n"),
            Markets::ICP => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "ICP-USD" }"#.to_string()+"\n"),
            Markets::TRX => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "TRX-USD" }"#.to_string()+"\n"),
            Markets::XTZ => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "XTZ-USD" }"#.to_string()+"\n"),
            Markets::INCH => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "1INCH-USD" }"#.to_string()+"\n"),
            Markets::SUSHI => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "SUSHI-USD" }"#.to_string()+"\n"),
            Markets::CELO => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "CELO-USD" }"#.to_string()+"\n"),
            Markets::UNI => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "UNI-USD" }"#.to_string()+"\n"),
            Markets::ETC => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "ETC-USD" }"#.to_string()+"\n"),
            Markets::BCH => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "BCH-USD" }"#.to_string()+"\n"),
            Markets::LTC => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "LTC-USD" }"#.to_string()+"\n"),
            Markets::XMR => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "XMR-USD" }"#.to_string()+"\n"),
            Markets::MKR => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "MKR-USD" }"#.to_string()+"\n"),
            Markets::UMA => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "UMA-USD" }"#.to_string()+"\n"),
            Markets::YFI => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "YFI-USD" }"#.to_string()+"\n"),
            Markets::RUNE => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "RUNE-USD" }"#.to_string()+"\n"),
            Markets::ADA => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "ADA-USD" }"#.to_string()+"\n"),
            Markets::ZEC => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "ZEC-USD" }"#.to_string()+"\n"),
            Markets::XLM => Message::Text(r#"{ "type": "subscribe", "channel": "v3_trades", "id": "XLM-USD" }"#.to_string()+"\n"),
        }
    }

    pub(crate) fn decimals(&self) -> usize {
        match *self {
            Markets::BTC => 10,
            Markets::ETH => 9,
            Markets::SOL => 7,
            Markets::FIL => 7,
            Markets::AVAX => 7,
            Markets::MATIC => 6,
            Markets::NEAR => 6,
            Markets::CRV => 6,
            Markets::LINK => 7,
            Markets::SNX => 7,
            Markets::DOT => 7,
            Markets::DOGE => 5,
            Markets::ATOM => 7,
            Markets::ENJ => 6,
            Markets::AAVE => 8,
            Markets::COMP => 8,
            Markets::ALGO => 6,
            Markets::ZRX => 6,
            Markets::EOS => 6,
            Markets::ICP => 7,
            Markets::TRX => 4,
            Markets::XTZ => 6,
            Markets::INCH => 7,
            Markets::SUSHI => 7,
            Markets::CELO => 6,
            Markets::UNI => 7,
            Markets::ETC => 7,
            Markets::BCH => 8,
            Markets::LTC => 8,
            Markets::XMR => 8,
            Markets::MKR => 9,
            Markets::UMA => 7,
            Markets::YFI => 10,
            Markets::RUNE => 6,
            Markets::ADA => 6,
            Markets::ZEC => 8,
            Markets::XLM => 5,
        }
    }

    pub(crate) fn id(&self) -> U512 {
        match *self {
            Markets::BTC => *BTC_ID,
            Markets::ETH => *ETH_ID,
            Markets::SOL => *SOL_ID,
            Markets::FIL => *FIL_ID,
            Markets::AVAX => *AVAX_ID,
            Markets::MATIC => *MATIC_ID,
            Markets::NEAR => *NEAR_ID,
            Markets::CRV => *CRV_ID,
            Markets::LINK => *LINK_ID,
            Markets::SNX => *SNX_ID,
            Markets::DOT => *DOT_ID,
            Markets::DOGE => *DOGE_ID,
            Markets::ATOM => *ATOM_ID,
            Markets::ENJ => *ENJ_ID,
            Markets::AAVE => *AAVE_ID,
            Markets::COMP => *COMP_ID,
            Markets::ALGO => *ALGO_ID,
            Markets::ZRX => *ZRX_ID,
            Markets::EOS => *EOS_ID,
            Markets::ICP => *ICP_ID,
            Markets::TRX => *TRX_ID,
            Markets::XTZ => *XTZ_ID,
            Markets::INCH => *INCH_ID,
            Markets::SUSHI => *SUSHI_ID,
            Markets::CELO => *CELO_ID,
            Markets::UNI => *UNI_ID,
            Markets::ETC => *ETC_ID,
            Markets::BCH => *BCH_ID,
            Markets::LTC => *LTC_ID,
            Markets::XMR => *XMR_ID,
            Markets::MKR => *MKR_ID,
            Markets::UMA => *UMA_ID,
            Markets::YFI => *YFI_ID,
            Markets::RUNE => *RUNE_ID,
            Markets::ADA => *ADA_ID,
            Markets::ZEC => *ZEC_ID,
            Markets::XLM => *XLM_ID,
        }
    }

    pub fn vector() -> Vec<Self> {
        [
            Markets::BTC,
            Markets::ETH,
            Markets::SOL,
            Markets::FIL,
            Markets::AVAX,
            Markets::MATIC,
            Markets::NEAR,
            Markets::CRV,
            Markets::LINK,
            Markets::SNX,
            Markets::DOT,
            Markets::DOGE,
            Markets::ATOM,
            Markets::ENJ,
            Markets::AAVE,
            Markets::COMP,
            Markets::ALGO,
            Markets::ZRX,
            Markets::EOS,
            Markets::ICP,
            Markets::TRX,
            Markets::XTZ,
            Markets::INCH,
            Markets::SUSHI,
            Markets::CELO,
            Markets::UNI,
            Markets::ETC,
            Markets::BCH,
            Markets::LTC,
            Markets::XMR,
            Markets::MKR,
            Markets::UMA,
            Markets::YFI,
            Markets::RUNE,
            Markets::ADA,
            Markets::ZEC,
            Markets::XLM
        ].to_vec()
    }

    // I did not really put that much consideration into any of these except for ETH.
    pub fn exposure(&self, exposure: Exposure) -> f64 {
        match *self {
            Markets::BTC => {
                match exposure {
                    Exposure::Low => 5f64,
                    Exposure::Medium => 10f64,
                    Exposure::High => 20f64,
                }
            },
            Markets::ETH => {
                match exposure {
                    Exposure::Low => 0.04f64,
                    Exposure::Medium => 0.08f64,
                    Exposure::High => 0.15f64,
                }
            },
            Markets::SOL => {
                match exposure {
                    Exposure::Low => 0.2f64,
                    Exposure::Medium => 0.5f64,
                    Exposure::High => 1f64,
                }
            },
            Markets::FIL => {
                match exposure {
                    Exposure::Low => 0.2f64,
                    Exposure::Medium => 0.4f64,
                    Exposure::High => 1f64,
                }
            },
            Markets::AVAX => {
                match exposure {
                    Exposure::Low => 0.5f64,
                    Exposure::Medium => 1f64,
                    Exposure::High => 2f64,
                }
            },
            Markets::MATIC => {
                match exposure {
                    Exposure::Low => 0.02f64,
                    Exposure::Medium => 0.5f64,
                    Exposure::High => 0.1f64,
                }
            },
            Markets::NEAR => {
                match exposure {
                    Exposure::Low => 0.03f64,
                    Exposure::Medium => 0.05f64,
                    Exposure::High => 0.1f64,
                }
            },
            Markets::CRV => {
                match exposure {
                    Exposure::Low => 0.02f64,
                    Exposure::Medium => 0.05f64,
                    Exposure::High => 0.1f64,
                }
            },
            Markets::LINK => {
                match exposure {
                    Exposure::Low => 0.2f64,
                    Exposure::Medium => 0.3f64,
                    Exposure::High => 0.5f64,
                }
            },
            Markets::SNX => {
                match exposure {
                    Exposure::Low => 0.03f64,
                    Exposure::Medium => 0.05f64,
                    Exposure::High => 0.1f64,
                }
            },
            Markets::DOT => {
                match exposure {
                    Exposure::Low => 0.2f64,
                    Exposure::Medium => 0.3f64,
                    Exposure::High => 0.5f64,
                }
            },
            Markets::DOGE => {
                match exposure {
                    Exposure::Low => 0.0002f64,
                    Exposure::Medium => 0.0003f64,
                    Exposure::High => 0.001f64,
                }
            },
            Markets::ATOM => {
                match exposure {
                    Exposure::Low => 5f64,
                    Exposure::Medium => 10f64,
                    Exposure::High => 20f64,
                }
            },
            Markets::ENJ => {
                match exposure {
                    Exposure::Low => 0.2f64,
                    Exposure::Medium => 0.5f64,
                    Exposure::High => 1.2f64,
                }
            },
            Markets::AAVE => {
                match exposure {
                    Exposure::Low => 1f64,
                    Exposure::Medium => 2f64,
                    Exposure::High => 4f64,
                }
            },
            Markets::COMP => {
                match exposure {
                    Exposure::Low => 1f64,
                    Exposure::Medium => 2f64,
                    Exposure::High => 4f64,
                }
            },
            Markets::ALGO => {
                match exposure {
                    Exposure::Low => 0.005f64,
                    Exposure::Medium => 0.008f64,
                    Exposure::High => 0.013f64,
                }
            },
            Markets::ZRX => {
                match exposure {
                    Exposure::Low => 0.005f64,
                    Exposure::Medium => 0.008f64,
                    Exposure::High => 0.13f64,
                }
            },
            Markets::EOS => {
                match exposure {
                    Exposure::Low => 0.2f64,
                    Exposure::Medium => 0.4f64,
                    Exposure::High => 0.8f64,
                }
            },
            Markets::ICP => {
                match exposure {
                    Exposure::Low => 0.5f64,
                    Exposure::Medium => 0.8f64,
                    Exposure::High => 1f64,
                }
            },
            Markets::TRX => {
                match exposure {
                    Exposure::Low => 0.0004f64,
                    Exposure::Medium => 0.0008f64,
                    Exposure::High => 0.0012f64,
                }
            },
            Markets::XTZ => {
                match exposure {
                    Exposure::Low => 0.1f64,
                    Exposure::Medium => 0.2f64,
                    Exposure::High => 0.4f64,
                }
            },
            Markets::INCH => {
                match exposure {
                    Exposure::Low => 0.004f64,
                    Exposure::Medium => 0.008f64,
                    Exposure::High => 0.02f64,
                }
            },
            Markets::SUSHI => {
                match exposure {
                    Exposure::Low => 0.02f64,
                    Exposure::Medium => 0.04f64,
                    Exposure::High => 0.08f64,
                }
            },
            Markets::CELO => {
                match exposure {
                    Exposure::Low => 0.02f64,
                    Exposure::Medium => 0.04f64,
                    Exposure::High => 0.08f64,
                }
            },
            Markets::UNI => {
                match exposure {
                    Exposure::Low => 0.05f64,
                    Exposure::Medium => 0.15f64,
                    Exposure::High => 0.3f64,
                }
            },
            Markets::ETC => {
                match exposure {
                    Exposure::Low => 0.1f64,
                    Exposure::Medium => 0.2f64,
                    Exposure::High => 1f64,
                }
            },
            Markets::BCH => {
                match exposure {
                    Exposure::Low => 1f64,
                    Exposure::Medium => 2f64,
                    Exposure::High => 4f64,
                }
            },
            Markets::LTC => {
                match exposure {
                    Exposure::Low => 0.5f64,
                    Exposure::Medium => 1f64,
                    Exposure::High => 2f64,
                }
            },
            Markets::XMR => {
                match exposure {
                    Exposure::Low => 0.5f64,
                    Exposure::Medium => 1f64,
                    Exposure::High => 2f64,
                }
            },
            Markets::MKR => {
                match exposure {
                    Exposure::Low => 1f64,
                    Exposure::Medium => 2f64,
                    Exposure::High => 5f64,
                }
            },
            Markets::UMA => {
                match exposure {
                    Exposure::Low => 0.05f64,
                    Exposure::Medium => 0.07f64,
                    Exposure::High => 0.1f64,
                }
            },
            Markets::YFI => {
                match exposure {
                    Exposure::Low => 2f64,
                    Exposure::Medium => 4f64,
                    Exposure::High => 8f64,
                }
            },
            Markets::RUNE => {
                match exposure {
                    Exposure::Low => 0.04f64,
                    Exposure::Medium => 0.08f64,
                    Exposure::High => 0.1f64,
                }
            },
            Markets::ADA => {
                match exposure {
                    Exposure::Low => 0.04f64,
                    Exposure::Medium => 0.08f64,
                    Exposure::High => 0.1f64,
                }
            },
            Markets::ZEC => {
                match exposure {
                    Exposure::Low => 0.4f64,
                    Exposure::Medium => 0.8f64,
                    Exposure::High => 1.2f64,
                }
            },
            Markets::XLM => {
                match exposure {
                    Exposure::Low => 0.04f64,
                    Exposure::Medium => 0.08f64,
                    Exposure::High => 0.1f64,
                }
            },
        }
    }

    pub fn default_order_size(&self) -> f64 {
        match *self {
            Markets::BTC => 0.001,
            Markets::ETH => 0.01,
            Markets::SOL => 2.0,
            Markets::FIL => 5.0,
            Markets::AVAX => 5.0,
            Markets::MATIC => 10.0,
            Markets::NEAR => 1.0,
            Markets::CRV => 10.0,
            Markets::LINK => 10.0,
            Markets::SNX => 10.0,
            Markets::DOT => 10.0,
            Markets::DOGE => 50.0,
            Markets::ATOM => 1.0,
            Markets::ENJ => 1.0,
            Markets::AAVE => 0.3,
            Markets::COMP => 0.3,
            Markets::ALGO => 1.0,
            Markets::ZRX => 10.0,
            Markets::EOS => 5.0,
            Markets::ICP => 5.0,
            Markets::TRX => 20.0,
            Markets::XTZ => 5.0,
            Markets::INCH => 5.0,
            Markets::SUSHI => 5.0,
            Markets::CELO => 10.0,
            Markets::UNI => 5.0,
            Markets::ETC => 2.0,
            Markets::BCH => 0.4,
            Markets::LTC => 0.5,
            Markets::XMR => 0.5,
            Markets::MKR => 0.1,
            Markets::UMA => 2.0,
            Markets::YFI => 0.01,
            Markets::RUNE => 10.0,
            Markets::ADA => 10.0,
            Markets::ZEC => 1.0,
            Markets::XLM => 10.0,
        }
    }

    pub fn dip_delta(&self) -> f64 {
        match *self {
            Markets::BTC => 100.0,
            Markets::ETH => 30.0,
            Markets::SOL => 2.0,
            Markets::FIL => 1.0,
            Markets::AVAX => 1.0,
            Markets::MATIC => 0.5,
            Markets::NEAR => 0.5,
            Markets::CRV => 0.3,
            Markets::LINK => 0.5,
            Markets::SNX => 0.1,
            Markets::DOT => 0.5,
            Markets::DOGE => 0.01,
            Markets::ATOM => 1.0,
            Markets::ENJ => 1.0,
            Markets::AAVE => 10.0,
            Markets::COMP => 10.0,
            Markets::ALGO => 1.0,
            Markets::ZRX => 0.5,
            Markets::EOS => 0.5,
            Markets::ICP => 0.5,
            Markets::TRX => 0.05,
            Markets::XTZ => 0.5,
            Markets::INCH => 1.0,
            Markets::SUSHI => 1.0,
            Markets::CELO => 1.0,
            Markets::UNI => 0.5,
            Markets::ETC => 1.0,
            Markets::BCH => 5.0,
            Markets::LTC => 5.0,
            Markets::XMR => 3.0,
            Markets::MKR => 10.0,
            Markets::UMA => 1.0,
            Markets::YFI => 50.0,
            Markets::RUNE => 1.0,
            Markets::ADA => 1.0,
            Markets::ZEC => 5.0,
            Markets::XLM => 0.5,
        }
    }

    pub fn price_delta(&self) -> f64 {
        match *self {
            Markets::BTC => 10.0,
            Markets::ETH => 5.0,
            Markets::SOL => 0.5,
            Markets::FIL => 0.5,
            Markets::AVAX => 0.5,
            Markets::MATIC => 0.2,
            Markets::NEAR => 0.2,
            Markets::CRV => 0.1,
            Markets::LINK => 0.2,
            Markets::SNX => 0.05,
            Markets::DOT => 0.2,
            Markets::DOGE => 0.002,
            Markets::ATOM => 0.5,
            Markets::ENJ => 0.5,
            Markets::AAVE => 3.0,
            Markets::COMP => 3.0,
            Markets::ALGO => 0.4,
            Markets::ZRX => 0.2,
            Markets::EOS => 0.2,
            Markets::ICP => 0.2,
            Markets::TRX => 0.02,
            Markets::XTZ => 0.2,
            Markets::INCH => 0.3,
            Markets::SUSHI => 0.3,
            Markets::CELO => 0.3,
            Markets::UNI => 0.2,
            Markets::ETC => 0.3,
            Markets::BCH => 2.0,
            Markets::LTC => 2.0,
            Markets::XMR => 1.0,
            Markets::MKR => 2.0,
            Markets::UMA => 0.5,
            Markets::YFI => 5.0,
            Markets::RUNE => 0.5,
            Markets::ADA => 0.5,
            Markets::ZEC => 2.0,
            Markets::XLM => 0.2,
        }
    }


}

impl fmt::Display for Markets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == Markets::INCH { write!(f, "1{:?}-USD", self) }
        else { write!(f, "{:?}-USD", self) }
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
