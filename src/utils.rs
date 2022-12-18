use ethers::prelude::*;
use std::sync::Arc;
use eyre::Result;
use std::num::ParseIntError;

/// Return a Provider for the given Websocket URL
pub async fn get_ws_provider() -> Result<Provider<Ws>> {
    Provider::<Ws>::connect("wss://arb-mainnet.g.alchemy.com/v2/GP1Ua7P55MXH1lWYfMg0kavjWXMVNGBx")
        .await
        .map_err(|e| eyre::eyre!("RPC Connection Error: {:?}", e))
}

/// Create Websocket Client
pub async fn create_websocket_client() -> Result<Arc<Provider<Ws>>> {
    let client = get_ws_provider().await?;
    Ok(Arc::new(client))
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}