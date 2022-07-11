extern crate dotenv;

use std::collections::HashMap;
use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;

struct Bot {
    exchange: dyn Exchange,
    // strategy: Strategy,
    // markets: Market[],
    // config: Config,
}

trait Exchange {    
    fn authenticate(&self) -> Result<(), ()>;
    // async fn get_candle_data(&self) -> Result<(), Error>;
    // async fn get_balances(&self) -> Result<f32, Box<dyn Error>>;
    // async fn get_market_price(&self) -> Result<f32, Box<dyn Error>>;
    // async fn place_sell_order(&self, amount: f32) -> Result<f32, Box<dyn Error>>;
    // async fn place_buy_order(&self, amount: f32) -> Result<f32, Box<dyn Error>>;
}

struct Kine {
}

impl Exchange for Kine {
    fn authenticate(&self) -> Result<(), ()> {
        Ok(())
    }
}

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    IUniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
    ]"#,
);

abigen!(
    Vault,
    r#"[
        function poolAmounts(address _token) external view returns (uint256)
    ]"#,
);

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv().ok();

    let client = Provider::<Http>::try_from(
        env::var("ARB_MAIN")?
    )?;
    let client = Arc::new(client);

    let contract_address = env::var("GMX_VAULT_MAIN")?.parse::<Address>()?;
    let contract = Vault::new(contract_address, Arc::clone(&client));

    let token_address = "0xff970a61a04b1ca14834a43f5de4533ebddb5cc8".parse::<Address>()?;

    // getReserves -> get_reserves
    let amt = contract.pool_amounts(token_address).call().await?;
    println!("{}", amt);
    Ok(())
}
