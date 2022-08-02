pub mod contract;
mod error;
pub mod helpers;
pub mod integration_tests;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// `period` is the locking period in **seconds** and `weight` is used to
/// calculate the amount of tokens returned. For example, if the weight is 0.25
/// and the deposited amount is 100DENOM, then 25uDENOM (100*0.25) is returned.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PeriodWeight {
    period: u128,
    weight: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum VestingPeriod {
    T1,
    T2,
    T3,
    T4,
}
