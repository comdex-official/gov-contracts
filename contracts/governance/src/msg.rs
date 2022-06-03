use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Coin, Timestamp};

use cw3::{Status, Vote};
use cw_utils::{ Expiration, Threshold, Duration};
use comdex_bindings::{ComdexMessages,ComdexQuery};
use crate::state::{Votes};
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InstantiateMsg {
    pub threshold: Threshold,
    
    pub target:String,
   
}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Voter {
    pub addr: String,
    pub weight: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]

pub struct ProposalResponseTotal{
    pub id :u64,
    pub title: String,
    pub start_time:Timestamp,
    pub description: String,
    pub start_height: u64,
    pub expires: Expiration,
    pub msgs: Vec<ComdexMessages>,
    pub status: Status,
    pub duration :Duration,
    /// pass requirements
    pub threshold: Threshold,
    // the total weight when the proposal started (used to calculate percentages)
    pub total_weight: u128,
    // summary of existing votes
    pub votes: Votes,
    pub proposer : String,
    pub token_denom :String,
    pub current_deposit:u128
 
}
// TODO: add some T variants? Maybe good enough as fixed Empty for now
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Propose {
        title: String,
        description: String,
        msgs: Vec<ComdexMessages>,
        // note: we ignore API-spec'd earliest if passed, always opens immediately
        latest: Option<Expiration>,
        app_id :u64
    },
    Vote {
        proposal_id: u64,
        vote: Vote,
    },
    Execute {
        proposal_id: u64,
    },
    Refund {
        proposal_id: u64,
    },
    Test {
        msg:ComdexMessages
    },
    Deposit {
        proposal_id: u64,
    }
}

// We can also add this as a cw3 extension
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Return ThresholdResponse
    Threshold {proposal_id: u64},
    /// Returns ProposalResponse
    Proposal { proposal_id: u64 },
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
    Vote { proposal_id: u64, voter: String },
    /// Returns VoteListResponse
    ListVotes {
        proposal_id: u64,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    ListAppProposal {
        app_id: u64,
    },

    Test{ query:ComdexQuery}
    ,
    AppAllUpData {
        app_id: u64,
    }
}
