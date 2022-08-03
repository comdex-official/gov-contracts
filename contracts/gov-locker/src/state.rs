use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// `period` is the locking period in **seconds** and `weight` is used to
/// calculate the amount of tokens returned. For example, if the weight is 0.25
/// and the deposited amount is 100DENOM, then 25uDENOM (100*0.25) is returned.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PeriodWeight {
    pub period: u128,
    pub weight: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
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
pub struct Vtoken {
    /// amount of token being locked
    pub token: u64,
    /// amount of vtoken released
    pub vtoken: u64,
    /// Locking period i.e. T1..4
    pub period: LockingPeriod,
    /// Time at which the tokens were locked
    pub start_time: SystemTime,
    /// Point in time after which the tokens can be unlocked
    pub end_time: SystemTime,
    /// Current status of the tokens
    pub status: Status,
}

/// NFT struct for holding the token info
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    /// Owner of the NFT
    pub owner: Addr,
    /// vtokens issued
    pub vtokens: Vec<Vtoken>,
    pub token_id: u64,
}

/// Contains the four locking periods and the unlock period.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub t1: PeriodWeight,
    pub t2: PeriodWeight,
    pub t3: PeriodWeight,
    pub t4: PeriodWeight,
    pub unlock_period: u128,
}

pub const STATE: Item<State> = Item::new("state");
pub const VTOKENS: Map<(Addr, u64), TokenInfo> = Map::new("vtokens");
