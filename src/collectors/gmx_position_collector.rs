use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use ethers::prelude::*;
use std::sync::Arc;
use crate::bindings::pos_router::{PositionRouterEvents, POS_ROUTER_ADDR};

/// A collector that listens for position changes on GMX, and generates a stream of
/// [events](GMXPosition) which contain trader address, collateral and other position info.
pub struct GMXPositionCollector<M> {
    client: Arc<M>,
}

impl<M> GMXPositionCollector<M> {
    pub fn new(client: Arc<M>) -> Self {
        Self { client }
    }
}

// A position change event on GMX
#[derive(Debug, Clone)]
pub struct GMXPosition {
    pub trader: Address,
    pub direction: PositionDirection,
    pub pos_type: PositionType,
}

#[derive(Debug, Clone)]
pub enum PositionDirection {
    Short,
    Long,
}

#[derive(Debug, Clone)]
pub enum PositionType {
    Increase,
    Decrease,
}

/// Implementation of the [Collector](Collector) trait for the [OpenseaOrderCollector](OpenseaOrderCollector).
#[async_trait]
impl<M> Collector<GMXPosition> for GMXPositionCollector<M>
where
    M: Middleware,
    M::Provider: PubsubClient,
    M::Error: 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, GMXPosition>> {
        let filter = Filter::new().address(
            POS_ROUTER_ADDR
                .parse::<Address>()
                .unwrap(),
        );
        let stream = self.client.subscribe_logs(&filter).await?;
        let stream = stream.filter_map(|log| async {
            match parse_log::<PositionRouterEvents>(log) {
                Ok(PositionRouterEvents::CreateIncreasePositionFilter(decoded)) => Some(GMXPosition {
                    trader: decoded.account,
                    pos_type: PositionType::Increase,
                    direction: if decoded.is_long {
                        PositionDirection::Long
                    } else {
                        PositionDirection::Short
                    },
                }),
                Ok(PositionRouterEvents::CreateDecreasePositionFilter(decoded)) => Some(GMXPosition {
                    trader: decoded.account,
                    pos_type: PositionType::Decrease,
                    direction: if decoded.is_long {
                        PositionDirection::Long
                    } else {
                        PositionDirection::Short
                    },
                }),
                _ => None,
            }
        });
        Ok(Box::pin(stream))
    }
}
