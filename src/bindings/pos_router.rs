use ethers::prelude::abigen;

abigen!(PositionRouter, "./src/bindings/pos_router_abi.json");

pub const POS_ROUTER_ADDR: &str = "0xb87a436B93fFE9D75c5cFA7bAcFff96430b09868";