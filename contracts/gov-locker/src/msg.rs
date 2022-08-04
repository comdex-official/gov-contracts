use crate::state::{LockingPeriod, PeriodWeight, Vtoken};
use cosmwasm_std::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Contains four locking periods and single unlock period. Each entry for t_i
/// is tuple consistings of lock-in period and the weightage.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub t1: PeriodWeight,
    pub t2: PeriodWeight,
    pub t3: PeriodWeight,
    pub t4: PeriodWeight,
    pub unlock_period: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Lock the amount of tokens for the given time period
    Lock {
        app_id: u64,
        locking_period: LockingPeriod,
    },
    /// Unlocks the locked tokens after meeting certain criteria
    Unlock {
        app_id: u64,
        denom:String
    },
     /// Withdraws the locked tokens after meeting certain criteria
    Withdraw{
        app_id:u64,
        denom:String,
        amount:u64
    }

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Query the amount of vTokens issued
    IssuedTokens { address: String, token_id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IssuedTokensResponse {
    pub vtokens: Vec<Vtoken>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct In {
    pub _vperiods: Vec<LockingPeriod>,
}
