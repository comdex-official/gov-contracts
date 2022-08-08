use cosmwasm_std::{Addr, Coin, Decimal, Timestamp, MessageInfo};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use std::time::SystemTime;

/// `period` is the locking period in **seconds** and `weight` is used to
/// calculate the amount of tokens returned. For example, if the weight is 0.25
/// and the deposited amount is 100DENOM, then 25uDENOM (100*0.25) is returned.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PeriodWeight {
    pub period: u64,
    pub weight: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LockingPeriod {
    T1,
    T2,
    T3,
    T4,
}

/// Holds the status of currently locked tokens. *Locked* means the tokens are
/// in the vesting period and *Released* means the tokens have completed their
/// vesting period and have been unlocked.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// When the tokens are in the vesting period.
    Locked,
    /// When the tokens have completed the locking period(T1..4) but not the
    /// unlock period (State.unlock_period). The owner has to wait for the
    /// unlock period to be over, before retrieving their tokens.
    Unlocking,
    /// When the token have completed both the locking period and the unlock
    /// period. The owner is free to retrieve their tokens.
    Unlocked,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CallType{
    Lock,

    Deposit,

    UpdateAmount,

    UpdateLokingPeriod,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Vtoken {
    /// amount of token being locked
    pub token: Coin,
    /// amount of vtoken created
    pub vtoken: Coin,
    /// Locking period i.e. T1..4
    pub period: Vec<LockingPeriodOfTokens>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LockingPeriodOfTokens{
    /// Locking period i.e. T1..4
    pub period: LockingPeriod,
    /// Time at which the tokens were locked
    pub start_time: Timestamp,
    /// Point in time after which the tokens can be unlocked
    pub end_time: Timestamp,
    /// Current status of the tokens
    pub status: Status,
}

/// NFT struct for holding the token info
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TokenInfo {
    /// Owner of the NFT
    pub owner: Addr,
    /// vtokens issued
    pub vtokens: Vec<Vtoken>,
    pub token_id: u64,
}

/// Contains the four locking periods and the unlock period.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct State {
    pub t1: PeriodWeight,
    pub t2: PeriodWeight,
    pub t3: PeriodWeight,
    pub t4: PeriodWeight,
    pub unlock_period: u64,
    pub num_tokens: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TokenSupply {
    // total token locked, unlocked or unlocking
    pub token: u128,
    // total vtoken released
    pub vtoken: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct LockNFTArgs{
    app_id:u64,
    locking_period:LockingPeriod,
    info:MessageInfo
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DepositArgs{
    app_id:u64,
    locking_period:LockingPeriod,
    info:MessageInfo
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct UpdateAmount{
    app_id:u64,
    locking_period:LockingPeriod,
    info:MessageInfo
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct UpdateLokingPeriod{
    app_id:u64,
    locking_period:LockingPeriod,
    info:MessageInfo
}




// The following mappings are as follows
// Holds the internal state
pub const STATE: Item<State> = Item::new("state");
// Owner to NFT
pub const TOKENS: Map<Addr, TokenInfo> = Map::new("tokens");
// Owner to Locked tokens
pub const LOCKED: Map<Addr, Vec<Coin>> = Map::new("locked");
// Owner to unlocking tokens, i.e. tokens in unlock_period
pub const UNLOCKING: Map<Addr, Vec<Coin>> = Map::new("unlocking");
// Owner to unlocked tokens, i.e. token that can be withdrawn
pub const UNLOCKED: Map<Addr, Vec<Coin>> = Map::new("unlocked");
// Total supply of each (vtoken supplied, token deposited)
pub const SUPPLY: Map<&str, TokenSupply> = Map::new("supply");
// Vtoken owned by an address for a specific denom
// !------- Should be Coin rather than Vtoken -------!
pub const VTOKENS: Map<(Addr, &str), Vtoken> = Map::new("Vtokens by NFT");
