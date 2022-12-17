extern crate dotenv;

use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;
use std::env;
use std::sync::Arc;
use ethers::{abi::AbiDecode, utils::keccak256};
use hex::FromHex;

pub mod utils;
// pub mod model;
// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    OrderBook,
    r#"[
        event CreateIncreaseOrder(address indexed account, uint256 orderIndex, address purchaseToken, uint256 purchaseTokenAmount, address collateralToken, address indexToken, uint256 sizeDelta, bool isLong, uint256 triggerPrice, bool triggerAboveThreshold, uint256 executionFee)
        event CreateDecreaseOrder(address indexed account,uint256 orderIndex,address collateralToken, uint256 collateralDelta, address indexToken, uint256 sizeDelta, bool isLong, uint256 triggerPrice, bool triggerAboveThreshold, uint256 executionFee)
    ]"#,
);

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv().ok();
    // Connect to Provider
    let client = utils::create_websocket_client().await?;

    // Orderbook contract
    let orderbook_contract = OrderBook::new(
        "0x09f77E8A13De9a35a7231028187e9fD5DB8a2ACB".parse::<Address>()?,
        client.clone(),
    );

    println!("Starting to listen to events....");
    // Subscribe to CreateIncreaseOrder events
    let events = orderbook_contract.events();
    let mut stream = events.subscribe().await?;

    loop {
        let next_item = stream.next().await.unwrap();
        let events = match next_item {
            Ok(data) => data,
            Err(e) => {
                //INFO: Error is usually a decoding error due to InvalidData. Dunno why
                // If the stream errors, we retry
                println!("Retrying stream...");
                continue;
            },
        };
        println!(
            "from: {:?}",
            events,
        );
    }

    Ok(())
    
        }