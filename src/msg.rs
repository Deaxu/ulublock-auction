use cosmwasm_std::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct InstantiateMsg {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    StartAuction {
        name: String,
        start_timestamp: u64,
        duration: u64,
        min_price: Uint128,
    },
    PlaceBid {
        auction_id: u64,
        bid_amount: Uint128,
    },
    EndAuction {
        auction_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AuctionDetails {
        auction_id: u64,
    },
    ListAuctions {},
}
