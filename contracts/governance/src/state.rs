use std::ops::Mul;

use comdex_bindings::ComdexMessages;
use cosmwasm_std::{Addr, BlockInfo, Coin, Decimal, StdResult, Storage, Timestamp, Uint128};
use cw3::{Status, Vote};
use cw_storage_plus::{Item, Map};
use cw_utils::{Duration, Expiration, Threshold};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// we multiply by this when calculating needed_votes in order to round up properly
// Note: `10u128.pow(9)` fails as "u128::pow` is not yet stable as a const fn"
const PRECISION_FACTOR: u128 = 1_000_000_000;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Config {
    pub threshold: Threshold,
    pub locking_contract: Addr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
pub struct AppGovConfig {
    pub proposal_count: u64,

    pub current_supply: u128,

    pub active_participation_supply: u128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AppProposalConfig {
    pub proposal_id: u64,

    pub proposal: Proposal,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
#[serde(rename_all = "snake_case")]
pub struct TokenSupply {
    // total token in the system.
    pub token: u128,
    // total vtoken released, for the corresponding token, in the system
    pub vtoken: u128,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]

pub struct Proposal {
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
    pub deposit: Vec<Coin>,
    pub proposer: String,
    pub token_denom: String,
    pub min_deposit: Uint128,
    pub current_deposit: u128,
    pub app_mapping_id: u64,
    pub is_slashed: bool,
}

impl Proposal {
    /// current_status is non-mutable and returns what the status should be.
    /// (designed for queries)
    pub fn current_status(&self, block: &BlockInfo) -> Status {
        let mut status = self.status;

        if status == Status::Executed {
            status = Status::Executed;
        } else if status == Status::Pending && self.expires.is_expired(block) {
            status = Status::Rejected;
        } else if self.expires.is_expired(block) && self.is_passed(block) {
            status = Status::Passed;
        } else if self.expires.is_expired(block) && self.is_rejected(block) {
            status = Status::Rejected;
        }

        status
    }

    /// update_status sets the status of the proposal to current_status.
    /// (designed for handler logic)
    pub fn update_status(&mut self, block: &BlockInfo) {
        self.status = self.current_status(block);
    }

    /// Returns true if this proposal is sure to pass (even before expiration, if no future
    /// sequence of possible votes could cause it to fail).
    pub fn is_passed(&self, _block: &BlockInfo) -> bool {
        match self.threshold {
            Threshold::AbsoluteCount {
                weight: weight_needed,
            } => self.votes.yes >= weight_needed,
            Threshold::AbsolutePercentage {
                percentage: percentage_needed,
            } => {
                self.votes.yes
                    >= votes_needed(self.total_weight - self.votes.abstain, percentage_needed)
            }
            Threshold::ThresholdQuorum { threshold, quorum } => {
                // we always require the quorum
                if self.votes.total() < votes_needed(self.total_weight, quorum)
                    || self.votes.total() == self.votes.abstain
                    || self.votes.veto
                        > (Decimal::percent(33) * Uint128::from(self.votes.total())).u128()
                {
                    false
                } else {
                    // If expired, we compare vote_count against the total number of votes (minus abstain).
                    let opinions = self.votes.total() - self.votes.abstain;
                    self.votes.yes >= votes_needed(opinions, threshold)
                }
            }
        }
    }

    pub fn is_rejected(&self, block: &BlockInfo) -> bool {
        match self.threshold {
            Threshold::AbsoluteCount {
                weight: weight_needed,
            } => {
                let weight = self.total_weight - weight_needed;
                self.votes.no > weight
            }
            Threshold::AbsolutePercentage {
                percentage: percentage_needed,
            } => {
                self.votes.no
                    > votes_needed(
                        self.total_weight - self.votes.abstain,
                        Decimal::one() - percentage_needed,
                    )
            }
            Threshold::ThresholdQuorum { threshold, quorum } => {
                let opinions = self.votes.total() - self.votes.abstain;

                if self.votes.total() < votes_needed(self.total_weight, quorum)
                    || self.votes.total() == self.votes.abstain
                    || self.votes.veto
                        > (Decimal::percent(33) * Uint128::from(self.votes.total())).u128()
                    || self.votes.yes <= votes_needed(opinions, threshold)
                {
                    true
                } else if self.expires.is_expired(block) {
                    // If expired, we compare vote_count against the total number of votes (minus abstain).
                    let opinions = self.votes.total() - self.votes.abstain;
                    self.votes.no > votes_needed(opinions, Decimal::one() - threshold)
                } else {
                    // If not expired, we must assume all non-votes will be cast for
                    let possible_opinions = self.total_weight - self.votes.abstain;
                    self.votes.no > votes_needed(possible_opinions, Decimal::one() - threshold)
                }
            }
        }
    }

    pub fn check_vetoed(&self, _block: &BlockInfo) -> bool {
        match self.threshold {
            Threshold::AbsoluteCount {
                weight: weight_needed,
            } => {
                let weight = self.total_weight - weight_needed;
                self.votes.no > weight
            }
            Threshold::AbsolutePercentage {
                percentage: percentage_needed,
            } => {
                self.votes.no
                    > votes_needed(
                        self.total_weight - self.votes.abstain,
                        Decimal::one() - percentage_needed,
                    )
            }
            Threshold::ThresholdQuorum {
                threshold: _,
                quorum,
            } => {
                self.votes.total() > votes_needed(self.total_weight, quorum)
                    && self.votes.veto
                        > (Decimal::percent(33).mul(Uint128::from(self.votes.total()))).u128()
            }
        }
    }
}

// weight of votes for each option
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
pub struct Votes {
    pub yes: u128,
    pub no: u128,
    pub abstain: u128,
    pub veto: u128,
}

impl Votes {
    /// sum of all votes
    pub fn total(&self) -> u128 {
        self.yes + self.no + self.abstain + self.veto
    }

    /// create it with a yes vote for this much
    pub fn yes(init_weight: u128) -> Self {
        Votes {
            yes: init_weight,
            no: 0,
            abstain: 0,
            veto: 0,
        }
    }

    pub fn add_vote(&mut self, vote: Vote, weight: u128) {
        match vote {
            Vote::Yes => self.yes += weight,
            Vote::Abstain => self.abstain += weight,
            Vote::No => self.no += weight,
            Vote::Veto => self.veto += weight,
        }
    }

    pub fn subtract_vote(&mut self, vote: Vote, weight: u128) {
        match vote {
            Vote::Yes => self.yes -= weight,
            Vote::Abstain => self.abstain -= weight,
            Vote::No => self.no -= weight,
            Vote::Veto => self.veto -= weight,
        }
    }
}

// this is a helper function so Decimal works with u64 rather than Uint128
// also, we must *round up* here, as we need 8, not 7 votes to reach 50% of 15 total
pub fn votes_needed(weight: u128, percentage: Decimal) -> u128 {
    let applied = percentage * Uint128::new(PRECISION_FACTOR * weight as u128);
    // Divide by PRECISION_FACTOR, rounding up to the nearest integer
    ((applied.u128() + PRECISION_FACTOR - 1) / PRECISION_FACTOR) as u128
}

// we cast a ballot with our chosen vote and a given weight
// stored under the key that voted
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Ballot {
    pub weight: u128,
    pub vote: Vote,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct VoteWeight {
    pub yes: u128,
    pub no: u128,
    pub abstain: u128,
    pub veto: u128,
}

// unique items
pub const CONFIG: Item<Config> = Item::new("config");
pub const PROPOSAL_COUNT: Item<u64> = Item::new("proposal_count");

// multiple-item map
pub const BALLOTS: Map<(u64, &Addr), Ballot> = Map::new("ballots");
pub const PROPOSALSBYAPP: Map<u64, Vec<u64>> = Map::new("proposals_by_app");
pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
pub const VOTERDEPOSIT: Map<(u64, &Addr), Vec<Coin>> = Map::new("voter_deposit");
pub const APPPROPOSALS: Map<u64, Vec<AppProposalConfig>> = Map::new("app_proposals");
pub const APPGOVCONFIG: Map<u64, AppGovConfig> = Map::new("app_gov_config");

pub fn next_id(store: &mut dyn Storage) -> StdResult<u64> {
    let id: u64 = PROPOSAL_COUNT.may_load(store)?.unwrap_or_default() + 1;
    PROPOSAL_COUNT.save(store, &id)?;
    Ok(id)
}
