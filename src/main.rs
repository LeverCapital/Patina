use std::collections::HashMap;

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

fn main() {
    println!("Hello, world!");
}
