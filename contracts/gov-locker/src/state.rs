use crate::PeriodWeight;
use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map, PrimaryKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy, JsonSchema)]
pub enum VestingPeriod {
    SHORT,
    MEDIUM,
    LONG,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy, JsonSchema)]
pub struct LockingPeriod {
    pub _type: VestingPeriod,
    pub _time: SystemTime,
    pub _weight: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Status {
    Locked,
    Released,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Locked {
    pub token: i64,
    pub value: i64,
    pub period: VestingPeriod,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
    pub status: Status,
}

pub const LOCKED: Item<Locked> = Item::new("Locked");
pub const VPERIOD: Map<i64, LockingPeriod> = Map::new("VestingPeriods");
pub const STATE: Item<State> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub t1: PeriodWeight,
    pub t2: PeriodWeight,
    pub t3: PeriodWeight,
    pub t4: PeriodWeight,
    pub unlock_period: u128,
}

// pub const STATE: Item<State> = Item::new("state");
