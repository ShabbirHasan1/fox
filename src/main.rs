#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use colored::Colorize;

use std::env;

use crate::dydx::{InternalAccount, Markets, Exposure};
use crate::strategy::Strategy;
use crate::analysis::*;

mod strategy;
mod dydx;
mod analysis;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let mut go = true;

    let mut stark_private_key: String = env::var("STARK_PRIVATE_KEY_MAINNET").expect("No STARK_PRIVATE_KEY found in .env or environment.");
    let mut api_key: String = env::var("API_KEY_MAINNET").expect("No API_KEY found in .env or environment.");
    let mut api_secret: String = env::var("API_SECRET_MAINNET").expect("No API_SECRET found in .env or environment.");
    let mut api_passphrase: String = env::var("API_PASSPHRASE_MAINNET").expect("No API_PASSPHRASE found in .env or environment.");
    let mut ethereum_address: String = env::var("ETHEREUM_ADDRESS_MAINNET").expect("No ETHEREUM_ADDRESS found in .env or environment.");
    
    let mut bot_strategy = Strategy::MarketMake;
    let mut exposure = Some(Exposure::High);
    let mut testnet = false;

    let arguments = env::args().collect::<Vec<String>>();
    let arg_help = arguments.iter().find(|&x| x == "-h" || x == "--help");
    let arg_strategy = arguments.iter().position(|x| x == "-s" || x == "--strategy");
    let arg_testnet = arguments.iter().position(|x| x == "--testnet");
    if arg_testnet.is_some() { 
        testnet = true; 
        stark_private_key = env::var("STARK_PRIVATE_KEY_TESTNET").expect("No STARK_PRIVATE_KEY found in .env or environment.");
        api_key = env::var("API_KEY_TESTNET").expect("No API_KEY found in .env or environment.");
        api_secret = env::var("API_SECRET_TESTNET").expect("No API_SECRET found in .env or environment.");
        api_passphrase = env::var("API_PASSPHRASE_TESTNET").expect("No API_PASSPHRASE found in .env or environment.");
        ethereum_address = env::var("ETHEREUM_ADDRESS_MAINNET").expect("No ETHEREUM_ADDRESS found in .env or environment.");
    }
    if arg_help.is_none() && arg_strategy.is_none() {
        println!("{} {}", "[-]".red().bold(), "No arguments specified. Defaulting to Strategy::MarketMake".red());
    }
    else if arg_strategy.is_some() {
        let arg_strategy = arg_strategy.unwrap();
        if arg_strategy == arguments.len()-1 { println!("{} {}", "[-]".red().bold(), "No strategy specified. Defaulting to Strategy::MarketMake".red()); }
        else {
            let strategy: &str = &arguments[arg_strategy+1];
            match strategy {
                "mm" => {
                    bot_strategy = Strategy::MarketMake;
                    println!("{} {}", "[+]".green().bold(), "Using Strategy::MarketMake".green());
                    exposure = Some(Exposure::Low);
                },
                "se" => {
                    bot_strategy = Strategy::StructuredEntropy;
                    println!("{} {}", "[+]".green().bold(), "Using Strategy::StructuredEntropy".green());
                },
                "sd" => {
                    bot_strategy = Strategy::SecondDerivative;
                    println!("{} {}", "[+]".green().bold(), "Using Strategy::SecondDerivative".green());
                }
                "gd" => {
                    bot_strategy = Strategy::ReverseRetail;
                    println!("{} {}", "[+]".green().bold(), "Using Strategy::ReverseRetail".green());
                }
                "me" => {
                    bot_strategy = Strategy::MarketExposure;
                    println!("{} {}", "[+]".green().bold(), "Using Strategy::MarketExposure".green());
                }
                _ => { println!("{} {}", "[-]".red().bold(), "Strategy not found. Defaulting to Strategy::MarketMake".red()); },
            }
        }
    }
    else {
        println!("Usage: ./fox -s <STRATEGY> || ./fox --strategy <STRATEGY>");
        go = false;
    }
    
    if go {
        let mut account_instance = InternalAccount::new_uninitialized(api_key, api_passphrase, api_secret, stark_private_key.clone(), ethereum_address);
        account_instance.onboard(testnet).await?;
        
        println!("{} {}{}{}{}", "[+]".green().bold(), "Internal account created with API key ".green(), account_instance.api_key().green(), " and ID ".green(), format!("{}", account_instance.position_id()).green());
        if bot_strategy != Strategy::MarketExposure { println!("{} {}", "[+]".yellow().bold(), "Initializing strategy. The bot takes ~10 minutes to gather data before it begins trading".yellow()); }
        else { println!("{} {}", "[+]".yellow().bold(), "Initializing strategy. The bot will need ~1 hour to gather data before it can trade".yellow()); }
        
        bot_strategy.run(account_instance, Markets::ETH, exposure, testnet).await?;

        /* // For debugging purposes
        let position = apis::dydx::Position::new("1500".to_string(), "0.01".to_string(), apis::dydx::Side::BUY);
        println!("{:?}", account_instance.open_order(dydx::Markets::ETH, position, apis::dydx::OrderType::LIMIT, "0.02".to_string(), "4".to_string(), 184552, false, None, None, None, false).await?);
        println!("{:?}", account_instance.cancel_order("3b30d3b2e239aad7233500f60fc50e2b31a466bb07afdd2280919f837932263".to_string(), false).await?);
        */
    }

    Ok(())
}
