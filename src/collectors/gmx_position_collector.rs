use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use ethers::prelude::*;
use std::sync::Arc;

abigen!(
    OrderBook,
    r#"[
        event CreateIncreaseOrder(address indexed account, uint256 orderIndex, address purchaseToken, uint256 purchaseTokenAmount, address collateralToken, address indexToken, uint256 sizeDelta, bool isLong, uint256 triggerPrice, bool triggerAboveThreshold, uint256 executionFee)
        event CreateDecreaseOrder(address indexed account,uint256 orderIndex,address collateralToken, uint256 collateralDelta, address indexToken, uint256 sizeDelta, bool isLong, uint256 triggerPrice, bool triggerAboveThreshold, uint256 executionFee)
    ]"#,
    derives(Copy)
);

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
    pub position_type: PositionType,
}

#[derive(Debug, Clone)]
pub enum PositionType {
    Short,
    Long,
}

// pub enum Position

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
            "0x09f77E8A13De9a35a7231028187e9fD5DB8a2ACB"
                .parse::<Address>()
                .unwrap(),
        );
        let stream = self.client.subscribe_logs(&filter).await?;
        let stream = stream.filter_map(|log| async {
            match parse_log::<OrderBookEvents>(log) {
                Ok(OrderBookEvents::CreateDecreaseOrderFilter(decoded)) => Some(GMXPosition {
                    trader: decoded.account,
                    position_type: PositionType::Short,
                }),
                Ok(OrderBookEvents::CreateIncreaseOrderFilter(decoded)) => Some(GMXPosition {
                    trader: decoded.account,
                    position_type: PositionType::Long,
                }),
                _ => None,
            }
        });
        Ok(Box::pin(stream))
    }
}
