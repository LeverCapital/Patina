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

abigen!(
    PositionRouter,
    r#"[
        event CreateIncreasePosition (address indexed account, address[] path, address indexToken, uint256 amountIn, uint256 minOut, uint256 sizeDelta, bool isLong, uint256 acceptablePrice, uint256 executionFee, uint256 index, uint256 queueIndex, uint256 blockNumber, uint256 blockTime, uint256 gasPrice)
        event CreateDecreasePosition (address indexed account, address[] path, address indexToken, uint256 collateralDelta, uint256 sizeDelta, bool isLong, address receiver, uint256 acceptablePrice, uint256 minOut, uint256 executionFee, uint256 index, uint256 queueIndex, uint256 blockNumber, uint256 blockTime)
    ]"#,
);

const POS_ROUTER_ADDR: &str = "0xb87a436B93fFE9D75c5cFA7bAcFff96430b09868";

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
                    position_type: PositionType::Short,
                }),
                Ok(PositionRouterEvents::CreateDecreasePositionFilter(decoded)) => Some(GMXPosition {
                    trader: decoded.account,
                    position_type: PositionType::Long,
                }),
                _ => None,
            }
        });
        Ok(Box::pin(stream))
    }
}
