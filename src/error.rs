use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuctionError {
    #[error("StdError: {0}")]
    Std(#[from] StdError),

    #[error("Auction is not enabled")]
    AuctionNotEnabled,

    #[error("Auction already exists with id: {0}")]
    AuctionAlreadyExists(u64),

    #[error("Auction not found")]
    AuctionNotFound,

    #[error("Bid amount must be greater than the current highest bid")]
    BidTooLow,

    #[error("Auction is not active")]
    AuctionNotActive,

    #[error("Auction has already been completed")]
    AuctionAlreadyCompleted,

    #[error("Invalid start timestamp. It must be greater than the current block time")]
    InvalidStartTimestamp,

    #[error("Invalid duration. It must be greater than 0")]
    InvalidDuration,

    #[error("Invalid auction duration. It must be between {min_duration} and {max_duration}")]
    InvalidAuctionDuration { min_duration: u64, max_duration: u64 },

    #[error("There is no winner for this auction")]
    NoWinner,

    #[error("There is no winner for this auction")]
    OwnerCannotBid

    #[error("Unauthorized")]
    Unauthorized,
}
