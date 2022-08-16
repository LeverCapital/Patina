extern crate dotenv;

use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

// Every bot runs one strategy on one exchange. This separates concerns.
// It can trade on multiple markets though
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

struct Kine {}

impl Exchange for Kine {
    fn authenticate(&self) -> Result<(), ()> {
        Ok(())
    }
}

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    CryptoKitty,
    r#"[
     isPregnant(uint256 _kittyId)(bool)
    ]"#,
);

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv().ok();
    // Setup bot's wallet
    let client = Arc::new({
        let provider = Provider::<Http>::try_from(env::var("ETH_MAIN")?)?;
        let chain_id = provider.get_chainid().await?;

        // this wallet's private key
        let wallet = env::var("BOT_PRIVATE_KEY")?
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        SignerMiddleware::new(provider, wallet)
    });

    let contract_address = "0x06012c8cf97BEaD5deAe237070F9587f8E7A266d".parse::<Address>()?;
    let contract = CryptoKitty::new(contract_address, Arc::clone(&client));

    let mut numPreggos = 0;
    let supply = 2013347;
    for n in 275495..supply {
        let index = n.to_string().parse::<U256>()?;
        let check = contract.is_pregnant(index).call().await?;
        if check {
            numPreggos += 1;
            println!("{}", n);
            if numPreggos == 22 {
                break;
            }
        }
    }

    Ok(())
}

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    PositionRouter,
    r#"[
        createIncreasePosition(address[] _path, address _indexToken, uint256 _amountIn, uint256 _minOut, uint256 _sizeDelta, bool _isLong, uint256 _acceptablePrice, uint256 _executionFee, bytes32 _referralCode)
    ]"#,
);

async fn increase_position(client: Arc<Provider<Http>>) -> Result<()> {
    let contract_address = env::var("GMX_POSROUTER_MAIN")?.parse::<Address>()?;
    let contract = PositionRouter::new(contract_address, Arc::clone(&client));
    Ok(())
}
