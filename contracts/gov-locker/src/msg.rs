use crate::state::{LockingPeriod, VestingPeriod};
use crate::PeriodWeight;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Contains four locking periods and single unlock period. Each entry for t_i
/// is tuple consistings of lock-in period and the weightage.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub t1: PeriodWeight,
    pub t2: PeriodWeight,
    pub t3: PeriodWeight,
    pub t4: PeriodWeight,
    pub unlock_period: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    SetPriod { _in: In },
    LockTokens { token: i64, value: i64, _type: i64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct In {
    pub _vperiods: Vec<LockingPeriod>,
}
