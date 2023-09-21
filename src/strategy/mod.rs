use crate::dydx::{InternalAccount, Markets, Exposure};

use crate::strategy::{
    market_make::MarketMake,
    second_derivative_trading::SecondDerivative,
    reverse_retail::ReverseRetail,
    structured_entropy::StructuredEntropy,
    market_exposure::MarketExposure
};

mod market_make;
mod second_derivative_trading;
mod structured_entropy;
mod reverse_retail;
mod market_exposure;

#[derive(Debug, PartialEq)]
pub enum Strategy {
    MarketMake,
    ReverseRetail,
    SecondDerivative,
    StructuredEntropy,
    MarketExposure,
}

impl Strategy {

    pub async fn run(&self, account: InternalAccount, market: Markets, exposure: Option<Exposure>, testnet: bool) -> anyhow::Result<()> {
        match *self {
            Strategy::MarketMake => {
                MarketMake::run(account, market, exposure.unwrap(), testnet).await?;
            },
            Strategy::SecondDerivative => {
                SecondDerivative::run(account, market, testnet).await?;
            }
            Strategy::MarketExposure => {
                MarketExposure::run(account, exposure.unwrap(), testnet).await?
            }
            Strategy::ReverseRetail => {
                ReverseRetail::run(account, market, exposure.unwrap(), testnet).await?
            }
            Strategy::StructuredEntropy => {
                StructuredEntropy::run(account, market, exposure.unwrap(), testnet).await?
            }
        }
        Ok(())
    }
}
