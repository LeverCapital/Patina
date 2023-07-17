use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use tracing::info;

use artemis_core::types::Strategy;
use ethers::{
    providers::{Middleware, StreamExt},
};

use crate::collectors::gmx_position_collector::GMXPosition;

use ethers::types::{H160};

/// Core Event enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Event {
    OpenPosition(GMXPosition),
    ClosePosition(GMXPosition),
}

/// Core Action enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Action {
    SubmitTx,
}

/// Configuration for variables we need to pass to the strategy.
#[derive(Debug, Clone)]
pub struct Config {
    pub arb_contract_address: H160,
    pub bid_percentage: u64,
}

#[derive(Debug, Clone)]
pub struct GMXCopyCatto<M> {
    /// Ethers client.
    client: Arc<M>,
}

impl<M: Middleware + 'static> GMXCopyCatto<M> {
    pub fn new(client: Arc<M>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for GMXCopyCatto<M> {
    // In order to sync this strategy, we need to get the current bid for all Sudo pools.
    async fn sync_state(&mut self) -> Result<()> {
        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::OpenPosition(position) => {
                info!("data: {:?}", position);
            }
            Event::ClosePosition(log) => info!("Close position: {:?}", log),
        }
        None
    }
}
