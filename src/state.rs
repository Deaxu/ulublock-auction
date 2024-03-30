use cosmwasm_std::{Addr, Timestamp, Storage};
use cosmwasm_storage::{singleton, singleton_read, Singleton};
use serde::{Serialize, Deserialize};

pub static AUCTIONS: Map<&str, Auction> = Map::new("auctions");
pub static CONFIG: Singleton<Config> = singleton("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub contract_name: String,
    pub contract_version: String,
    pub cw20_addr: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum AuctionStatus {
    Active,
    Completed,
    Cancelled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Bid {
    pub bidder: Addr,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Auction {
    pub id: u64,
    pub name: String,
    pub owner: Addr,
    pub start_timestamp: Timestamp,
    pub duration: u64,
    pub nft_contract_addr: Addr,
    pub token_id: String,
    pub min_price: Option<u128>,
    pub bids: Vec<Bid>, // Teklifleri saklamak için eklenen yeni alan
    pub highest_bidder: Option<Addr>,
    pub highest_bid: Option<u128>,
    pub status: AuctionStatus,
}

