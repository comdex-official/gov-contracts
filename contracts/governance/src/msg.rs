use crate::state::Votes;
use comdex_bindings::ComdexMessages;
use cosmwasm_std::{Decimal, Timestamp,Addr};
use cw3::{Status, Vote};
use cw_utils::{Duration, Expiration, Threshold};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};




#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InstantiateMsg {
    pub threshold: Threshold,
    pub locking_contract:Addr,
    pub target: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Voter {
    pub addr: String,
    pub weight: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]

pub struct ProposalResponseTotal {
    pub id: u64,
    pub title: String,
    pub start_time: Timestamp,
    pub description: String,
    pub start_height: u64,
    pub expires: Expiration,
    pub msgs: Vec<ComdexMessages>,
    pub status: Status,
    pub duration: Duration,
    /// pass requirements
    pub threshold: Threshold,
    // the total weight when the proposal started (used to calculate percentages)
    pub total_weight: u128,
    // summary of existing votes
    pub votes: Votes,
    pub proposer: String,
    pub token_denom: String,
    pub current_deposit: u128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Propose {
    pub title: String,
    pub description: String,
    pub msgs: Vec<ComdexMessages>,
    // note: we ignore API-spec'd earliest if passed, always opens immediately
    pub latest: Option<Expiration>,
    pub app_id_param: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ExtendedPair {
    pub app_mapping_id_param: u64,
    pub pair_id_param: u64,
    pub stability_fee_param: Decimal,
    pub closing_fee_param: Decimal,
    pub draw_down_fee_param: Decimal,
    pub debt_ceiling_param: u64,
    pub debt_floor_param: u64,
    pub pair_name_param: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Propose { propose: Propose },
    Vote { proposal_id: u64, vote: Vote },
    Execute { proposal_id: u64 },
    Refund { proposal_id: u64 },

    Deposit { proposal_id: u64 },
    Slash { proposal_id: u64 },
}

// We can also add this as a cw3 extension
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Return ThresholdResponse
    Threshold {
        proposal_id: u64,
    },
    /// Returns ProposalResponse
    Proposal {
        proposal_id: u64,
    },
    /// Returns ProposalListResponse
    ListProposals {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    /// Returns ProposalListResponse
    ReverseProposals {
        start_before: Option<u64>,
        limit: Option<u32>,
    },
    /// Returns VoteResponse
    Vote {
        proposal_id: u64,
        voter: String,
    },
    /// Returns VoteListResponse
    ListVotes {
        proposal_id: u64,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    ListAppProposal {
        app_id: u64,
    },

    AppAllUpData {
        app_id: u64,
    },
    Supply {
        denom: String,
    },
    TotalVTokens
    {
        address:Addr,
        denom:String,
        height: Option<u64>
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    UpdateLockingContract {
        address: Addr,
    },
    UpdateThreshold{threshold : Threshold},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
