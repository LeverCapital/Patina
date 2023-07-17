mod collectors;
mod strategies;

use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use clap::Parser;
use collectors::gmx_position_collector::GMXPositionCollector;
use ethers::providers::{Provider, Ws};
use ethers::signers::{LocalWallet, Signer};
use strategies::gmx_copy_catto::{Action, Event, GMXCopyCatto};
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};

use std::sync::Arc;

use artemis_core::engine::Engine;
use artemis_core::types::CollectorMap;

/// CLI Options.
#[derive(Parser, Debug)]
pub struct Args {
    /// Ethereum node WS endpoint.
    #[arg(long)]
    pub wss: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up tracing and parse args.
    let filter = filter::Targets::new()
        .with_target("gmx_copy_catto", Level::INFO)
        .with_target("artemis_core", Level::INFO);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let args = Args::parse();

    // Set up ethers provider.
    let ws = Ws::connect(args.wss).await?;
    let provider = Provider::new(ws);

    // Set up engine.
    let mut engine: Engine<Event, Action> = Engine::default();

    // Set up block collector.
    let gmx_collector = Box::new(GMXPositionCollector::new(provider.clone().into()));
    let gmx_collector = CollectorMap::new(gmx_collector, Event::OpenPosition);
    engine.add_collector(Box::new(gmx_collector));

    let strat = GMXCopyCatto::new(Arc::new(provider.clone()));
    engine.add_strategy(Box::new(strat));

    // Start engine.
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("res: {:?}", res);
        }
    }
    Ok(())
}
