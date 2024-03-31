// nft_contract_addr ve token_id alanları nasıl doldurulacak? (nft_contract_addr: NFT'nin kontrat adresi, token_id: NFT'nin token id'si)
// Cw20 tokenin kontrata gönderilmesi işlemi nasıl yapılacak?
// İade işlemleri ne zaman yapılacak? (Açık artırma bitince mi, yeni highest bid geldiğinde mi?)
// Açık artırmanın süresinin dolup dolmadığı mesajı kontrata nasıl gönderilecek (end_auction)?  

use cosmwasm_std::{
    entry_point, to_binary, Addr, BankMsg, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw721_base::{cw721_execute_msgs::TransferNft, Cw721Contract};
use crate::error::AuctionError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Auction, AuctionStatus, AUCTIONS, CONFIG, Config};
use cosmwasm_std::{CosmosMsg, WasmMsg};

// Kontratın sürüm bilgisi
// Henüz kontratı güncelleme işlevi olmadığı için bu bilgiye ihtiyacımız yok. Eklersek güzel olur.
const CONTRACT_NAME: &str = "nftlaunchpet-auction";
const CONTRACT_VERSION: &str = "0.0.1";

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: InstantiateMsg) -> Result<Response, AuctionError> {

    let config = Config {
        contract_name: CONTRACT_NAME.to_string(),
        contract_version: CONTRACT_VERSION.to_string(),
        cw20_addr: msg.cw20_addr, // CW20 token adresi eklenmeli
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, AuctionError> {
    match msg{
        ExecuteMsg::StartAuction { name, start_timestamp, duration, min_price } => {
            start_auction(deps, info, env, name, start_timestamp, duration, min_price)
        },
        ExecuteMsg::PlaceBid { auction_id, bid_amount } => {
            place_bid(deps, info, auction_id, bid_amount)
        },
        ExecuteMsg::EndAuction { auction_id } => {
            end_auction(deps, info, auction_id)
        },
    }
}

/*
!HENÜZ DENENMEDİ!
# Deploy
injectived tx wasm store contract.wasm --from <your-account> --chain-id="injective-888" --gas auto --gas-adjustment 1.4 --gas-prices "10inj" -y

# Instantiate
injectived tx wasm instantiate <code-id> '{"count": 100}' --from <your-account> --label "my first contract" --chain-id="injective-888" --amount 50000inj --gas auto --gas-adjustment 1.4 --gas-prices "10inj" -y

# Execute
injectived tx wasm execute <contract-address> '{"start_auction": {"name": "Auction 1", "start_timestamp": "1630000000", "nft_contract_addr": " ", "token_id": " ", "duration": "3600", "min_price": "1000000"}}' --from <your-account> --chain-id="injective-888" --gas auto --gas-prices "10inj" --broadcast-mode block

injectived tx wasm execute <contract-address> '{"place_bid": {"auction_id": "1", "bid_amount": "1500000"}}' --from <your-account> --chain-id="injective-888" --gas auto --gas-prices "10inj" --broadcast-mode block

 */

fn start_auction(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    name: String,
    start_timestamp: u64,
    nft_contract_addr: Addr,
    token_id: String,
    duration: u64,
    min_price: Uint128,
) -> Result<Response, AuctionError> {
    let config = CONFIG.load(deps.storage)?;

    let start_timestamp = Timestamp::from_seconds(start_timestamp);

    if start_timestamp <= env.block.time {
        return Err(AuctionError::InvalidStartTimestamp);
    }

    // NFT'yi kontrata gönder. Gerçekten çalışacak mı bu şekilde kontrol edilmeli. Örnekleri araştırılmalı.
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        msg: to_binary(&TransferNft {
            recipient: auction.contract_addr.to_string(),
            token_id: auction.token_id.clone(),
        })?,
        funds: vec![],
    }));

    let auction = Auction {
        id: AUCTIONS.count(deps.storage)? + 1,
        owner: info.sender.clone(),
        start_timestamp,
        duration,
        nft_contract_addr, // Bu alanın nasıl doldurulacağına dair mantık eklenmeli.
        token_id: String::new(), // Bu alanın nasıl doldurulacağına dair mantık eklenmeli.
        min_price: Some(min_price.u128()),
        bids: vec![],
        highest_bidder: None,
        highest_bid: None,
        status: AuctionStatus::Active,
    };

    AUCTIONS.save(deps.storage, &auction.id.to_string(), &auction)?;

    Ok(Response::new().add_attribute("method", "start_auction").add_attribute("auction_id", auction.id.to_string()))
}

fn place_bid(
    deps: DepsMut,
    info: MessageInfo,
    auction_id: u64,
    bid_amount: Uint128,
) -> Result<Response, AuctionError> {

    let mut auction = AUCTIONS.load(deps.storage, &auction_id.to_string())?;

    if auction.status != AuctionStatus::Active {
        return Err(AuctionError::AuctionNotActive);
    }

    if let Some(min_price) = auction.min_price {
        if bid_amount_value < min_price {
            return Err(AuctionError::BidTooLow);
        }
    }

    let bid_amount_value = bid_amount.u128();
    if let Some(highest_bid) = auction.highest_bid {
        if bid_amount_value <= highest_bid {
            return Err(AuctionError::BidTooLow);
        }
    }

    let bid = Bid {
        bidder: info.sender.clone(),
        amount: bid_amount,
    };

    // CW20 tokenin kontrata gönderilmesi işlemi. Gerçekten çalışacak mı bu şekilde kontrol edilmeli. Örnekleri araştırılmalı.
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: CONFIG.load(deps.storage)?.cw20_addr,
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: CONFIG.load(deps.storage)?.contract_addr,
            amount: bid_amount,
        })?,
    }));

    auction.bids.push(bid);
    auction.highest_bidder = Some(info.sender.clone());
    auction.highest_bid = Some(bid_amount_value);
    AUCTIONS.save(deps.storage, &auction_id.to_string(), &auction)?;

    Ok(Response::new().add_attribute("method", "place_bid"))
}

fn end_auction(
    deps: DepsMut,
    info: MessageInfo,
    auction_id: u64,
) -> Result<Response, AuctionError> {

    let auction = AUCTIONS.load(deps.storage, &auction_id.to_string())?;

    if auction.status != AuctionStatus::Active {
        return Err(AuctionError::AuctionNotActive);
    }

    if env.block.time < auction.start_timestamp.plus_seconds(auction.duration) {
        return Err(AuctionError::AuctionNotCompleted);
    }

    let mut messages: Vec<CosmosMsg> = vec![];

    if let Some(highest_bidder) = auction.highest_bidder {
        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: CONFIG.load(deps.storage)?.nft_addr,
            msg: to_binary(&TransferNft {
                recipient: highest_bidder.to_string(),
                token_id: auction.token_id.clone(),
            })?,
            funds: vec![],
        }));
    }

    // En yüksek teklif hariç diğer tüm teklif sahiplerine token iadeleri
    if let Some(_) = auction.highest_bid {
        for bid in auction.bids.iter().filter(|b| b.bidder != highest_bidder) {
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: CONFIG.load(deps.storage)?.cw20_addr,
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: bid.bidder.to_string(),
                    amount: bid.amount,
                })?,
                funds: vec![],
            }));
        }
    }

    // Auction verilerini güncelle ve sakla
    auction.status = AuctionStatus::Closed;
    AUCTIONS.save(deps.storage, &auction_id.to_string(), &auction)?;

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "close_auction"))
}










// Acelesi yok halledilir
/* 
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ListAuctions {} => list_auctions(deps),

        QueryMsg::AuctionDetails { auction_id } => {
            query_auction_details(deps, auction_id)
        },
    }
}*/
