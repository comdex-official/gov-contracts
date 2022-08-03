use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy, JsonSchema)]
// pub enum VestingPeriod {
//     SHORT,
//     MEDIUM,
//     LONG,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy, JsonSchema)]
// pub struct LockingPeriod {
//     pub _type: VestingPeriod,
//     pub _time: SystemTime,
//     pub _weight: i64,
// }

/// `period` is the locking period in **seconds** and `weight` is used to
/// calculate the amount of tokens returned. For example, if the weight is 0.25
/// and the deposited amount is 100DENOM, then 25uDENOM (100*0.25) is returned.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PeriodWeight {
    period: u128,
    weight: f64,
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
    Locked,
    Released,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TokenInfo {
    /// Owner of the NFT
    owner: Addr,
    /// Amount of vtokens issued
    vtokens: Vec<Vtoken>,
    token_id: u64,
}

// pub const LOCKED: Item<Locked> = Item::new("Locked");
// pub const VPERIOD: Map<i64, LockingPeriod> = Map::new("VestingPeriods");

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
