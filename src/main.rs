extern crate dotenv;

use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;

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
        let event = match next_item {
            Ok(data) => data,
            Err(_) => {
                // Error is usually due to decoding invalid data. Never mind, just retry
                println!("Retrying stream...");
                continue;
            },
        };
        match event {
            OrderBookEvents::CreateIncreaseOrderFilter(event) => {
                println!("CreateIncreaseOrder event received");
                println!("from: {:?}", event.account);
            },
            OrderBookEvents::CreateDecreaseOrderFilter(event) => {
                println!("CreateDecreaseOrder event received");
                println!("from: {:?}", event.account);
            },
        }

    }

    Ok(())
    
        }