use crate::coin_helpers::assert_sent_sufficient_coin_deposit;
use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, ExtendedPair, InstantiateMsg, ProposalResponseTotal, Propose, QueryMsg,
};
use crate::state::{
    next_id, AppGovConfig, Ballot, Config, Proposal, Votes, APPGOVCONFIG, APPPROPOSALS, BALLOTS,
    CONFIG, PROPOSALS, PROPOSALSBYAPP, VOTERDEPOSIT,
};
use crate::validation::{
    add_extended_pair_vault, auction_mapping_for_app, collector_lookup_table, get_token_supply,
    query_app_exists, query_get_asset_data, query_owner_token_at_height,
    remove_whitelist_app_id_liquidation, remove_whitelist_app_id_vault_interest,
    remove_whitelist_asset_locker, update_locker_lsr, update_pairvault_stability,
    validate_threshold, whitelist_app_id_liquidation, whitelist_app_id_vault_interest,
    whitelist_asset_locker_eligible, whitelist_asset_locker_rewards,
};
use comdex_bindings::{ComdexMessages, ComdexQuery};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, BlockInfo, Coin, Deps, DepsMut, Env, MessageInfo,
    Order, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw3::{
    ProposalListResponse, ProposalResponse, Status, Vote, VoteInfo, VoteListResponse, VoteResponse,
};
use cw_storage_plus::Bound;
use cw_utils::{Duration, Threshold, ThresholdResponse};
use std::cmp::Ordering;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:governance";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ComdexQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    //Only Quorum Threshold allowed for voting
    match msg.threshold {
        Threshold::AbsoluteCount { weight: _ } => {
            return Err(ContractError::AbsoluteCountNotAccepted {})
        }
        Threshold::AbsolutePercentage { percentage: _ } => {
            return Err(ContractError::AbsolutePercentageNotAccepted {})
        }
        Threshold::ThresholdQuorum { threshold, quorum } => {
            validate_threshold(&threshold, &quorum)?
        }
    }

    //// set contract version for migration
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let cfg = Config {
        threshold: msg.threshold,
        target: msg.target,
    };
    CONFIG.save(deps.storage, &cfg)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ComdexQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ComdexMessages>, ContractError> {
    match msg {
        ExecuteMsg::Propose { propose } => execute_propose(deps, env, info, propose),
        ExecuteMsg::Vote { proposal_id, vote } => execute_vote(deps, env, info, proposal_id, vote),
        ExecuteMsg::Execute { proposal_id } => execute_execute(deps, env, info, proposal_id),
        ExecuteMsg::Refund { proposal_id } => execute_refund(deps, env, info, proposal_id),
        ExecuteMsg::Deposit { proposal_id } => execute_deposit(deps, env, info, proposal_id),
        ExecuteMsg::Slash { proposal_id } => execute_slash(deps, env, info, proposal_id),
    }
}

pub fn execute_propose(
    deps: DepsMut<ComdexQuery>,
    env: Env,
    info: MessageInfo,
    propose: Propose,
) -> Result<Response<ComdexMessages>, ContractError> {
    // get proposal message length
    let msg_length = propose.msgs.len();

    //throw empty message error
    if propose.msgs.is_empty() {
        return Err(ContractError::NoMessage {});
    }

    // should be only 1 message
    if msg_length > 1 {
        return Err(ContractError::ExtraMessages {});
    }

    //get app data for app_id
    let app_response = query_app_exists(deps.as_ref(), propose.app_id)?;

    let voting_time = app_response.gov_time_in_seconds;
    let min_gov_deposit = app_response.min_gov_deposit;
    let max_voting_period = Duration::Time(voting_time);
    let gov_token_id = app_response.gov_token_id;

    //get gov token denom name
    let gov_token_denom = query_get_asset_data(deps.as_ref(), gov_token_id)?;
    if gov_token_denom.is_empty() || gov_token_id == 0 {
        return Err(ContractError::NoGovToken {});
    }

    //get total supply for denom to get proposal weight
    let total_weight = get_token_supply(deps.as_ref(), propose.app_id, gov_token_id)?;
    if total_weight == 0 {
        return Err(ContractError::ZeroSupply {});
    }

    let cfg = CONFIG.load(deps.storage)?;
    let height = env.block.height - 1;

    //Calculate proposer voting Power

    let voting_power = query_owner_token_at_height(
        deps.as_ref(),
        info.sender.to_string(),
        gov_token_denom.to_string(),
        height.to_string(),
        cfg.target,
    )?;

    // max expires also used as default
    let max_expires = max_voting_period.after(&env.block);
    let mut expires = propose.latest.unwrap_or(max_expires);
    let comp = expires.partial_cmp(&max_expires);
    if let Some(Ordering::Greater) = comp {
        expires = max_expires;
    } else if comp.is_none() {
        return Err(ContractError::WrongExpiration {});
    }

    //Check if no other deposit provided other than gov token deposit
    let funds_len = info.funds.len();

    if funds_len > 1 {
        return Err(ContractError::AdditionalDenomDeposit {});
    }

    //check if gov denom exists in user deposit

    let mut gov_current_deposit: u128 = 0;
    let mut is_correct_denom = false;
    for user_deposit in &info.funds {
        if user_deposit.denom.eq(&gov_token_denom) {
            is_correct_denom = true;
            gov_current_deposit = user_deposit.amount.u128();
        }
    }

    // return error if wrong denom deposit
    if !is_correct_denom {
        return Err(ContractError::DenomNotFound {});
    }

    //Handle execution messages

    for msg in propose.msgs.clone() {
        match msg {
            ComdexMessages::MsgWhiteListAssetLocker {
                app_mapping_id,
                asset_id,
            } => whitelist_asset_locker_eligible(
                deps.as_ref(),
                app_mapping_id,
                asset_id,
                propose.app_id,
            )?,
            ComdexMessages::MsgWhitelistAppIdLockerRewards {
                app_mapping_id,
                asset_id,
            } => whitelist_asset_locker_rewards(
                deps.as_ref(),
                app_mapping_id,
                asset_id,
                propose.app_id,
            )?,
            ComdexMessages::MsgWhitelistAppIdVaultInterest { app_mapping_id } => {
                whitelist_app_id_vault_interest(deps.as_ref(), app_mapping_id, propose.app_id)?
            }
            ComdexMessages::MsgAddExtendedPairsVault {
                app_mapping_id,
                pair_id,
                stability_fee,
                closing_fee,
                liquidation_penalty: _,
                draw_down_fee,
                is_vault_active: _,
                debt_ceiling,
                debt_floor,
                is_stable_mint_vault: _,
                min_cr: _,
                pair_name,
                asset_out_oracle_price: _,
                asset_out_price: _,
                min_usd_value_left: _,
            } => add_extended_pair_vault(
                deps.as_ref(),
                propose.app_id,
                ExtendedPair {
                    app_mapping_id_param: app_mapping_id,
                    pair_id_param: pair_id,
                    stability_fee_param: stability_fee,
                    closing_fee_param: closing_fee,
                    draw_down_fee_param: draw_down_fee,
                    debt_ceiling_param: debt_ceiling,
                    debt_floor_param: debt_floor,
                    pair_name_param: pair_name,
                },
            )?,
            ComdexMessages::MsgSetCollectorLookupTable {
                app_mapping_id,
                collector_asset_id,
                secondary_asset_id,
                surplus_threshold: _,
                debt_threshold: _,
                locker_saving_rate: _,
                lot_size: _,
                bid_factor: _,
                debt_lot_size: _,
            } => collector_lookup_table(
                deps.as_ref(),
                app_mapping_id,
                collector_asset_id,
                secondary_asset_id,
                propose.app_id,
            )?,

            ComdexMessages::MsgUpdatePairsVault {
                app_mapping_id,
                ext_pair_id,
                stability_fee: _,
                closing_fee: _,
                liquidation_penalty: _,
                draw_down_fee: _,
                min_cr: _,
                debt_ceiling: _,
                debt_floor: _,
                min_usd_value_left: _,
            } => update_pairvault_stability(
                deps.as_ref(),
                app_mapping_id,
                ext_pair_id,
                propose.app_id,
            )?,

            ComdexMessages::MsgSetAuctionMappingForApp {
                app_mapping_id,
                asset_id: _,
                is_surplus_auction: _,
                is_debt_auction: _,
                asset_out_oracle_price: _,
                asset_out_price: _,
            } => auction_mapping_for_app(deps.as_ref(), app_mapping_id, propose.app_id)?,

            ComdexMessages::MsgUpdateCollectorLookupTable {
                app_mapping_id,
                asset_id,
                lsr: _,
                debt_threshold: _,
                surplus_threshold: _,
                lot_size: _,
                debt_lot_size: _,
                bid_factor: _,
            } => update_locker_lsr(deps.as_ref(), app_mapping_id, asset_id, propose.app_id)?,
            ComdexMessages::MsgRemoveWhitelistAssetLocker {
                app_mapping_id,
                asset_id,
            } => remove_whitelist_asset_locker(
                deps.as_ref(),
                app_mapping_id,
                asset_id,
                propose.app_id,
            )?,
            ComdexMessages::MsgRemoveWhitelistAppIdVaultInterest { app_mapping_id } => {
                remove_whitelist_app_id_vault_interest(
                    deps.as_ref(),
                    app_mapping_id,
                    propose.app_id,
                )?
            }
            ComdexMessages::MsgWhitelistAppIdLiquidation { app_mapping_id } => {
                whitelist_app_id_liquidation(deps.as_ref(), app_mapping_id, propose.app_id)?
            }
            ComdexMessages::MsgRemoveWhitelistAppIdLiquidation { app_mapping_id } => {
                remove_whitelist_app_id_liquidation(deps.as_ref(), app_mapping_id, propose.app_id)?
            }
            ComdexMessages::MsgAddAuctionParams {
                app_mapping_id: _,
                auction_duration_seconds: _,
                buffer: _,
                cusp: _,
                step: _,
                price_function_type: _,
                surplus_id: _,
                debt_id: _,
                dutch_id: _,
                bid_duration_seconds: _,
            } => (),
            _ => return Err(ContractError::ProposalNotEligible {}),
        }
    }

    //check if coins deposited is sufficient to pass minimum deposit
    //if minimum deposit is achieved ,propsal status becomes "Open" else it becomes "Pending"
    let min_deposit = Coin {
        amount: Uint128::from(min_gov_deposit),
        denom: gov_token_denom.clone(),
    };
    let deposit_status = assert_sent_sufficient_coin_deposit(&info.funds, Some(min_deposit))?;

    // initialize a proposal
    let mut prop = Proposal {
        title: propose.title,
        description: propose.description,
        start_time: env.block.time,
        start_height: env.block.height,
        expires,
        msgs: propose.msgs,
        duration: max_voting_period,
        status: deposit_status,
        votes: Votes::yes(voting_power.amount.u128()),
        threshold: cfg.threshold,
        total_weight: Uint128::from(total_weight).u128(),
        deposit: info.funds.clone(),
        proposer: info.sender.to_string(),
        token_denom: gov_token_denom,
        min_deposit: min_gov_deposit,
        current_deposit: gov_current_deposit,
        app_mapping_id: propose.app_id,
        is_slashed: false,
    };

    //update proposal status
    prop.update_status(&env.block);

    //get proposals by app
    let mut app_proposals = match APPPROPOSALS.may_load(deps.storage, propose.app_id)? {
        Some(record) => record,
        None => vec![],
    };

    //get latest proposal id counter
    let id = next_id(deps.storage)?;

    // update proposals
    PROPOSALS.save(deps.storage, id, &prop)?;
    app_proposals.push(crate::state::AppProposalConfig {
        proposal_id: id,
        proposal: prop.clone(),
    });
    APPPROPOSALS.save(deps.storage, propose.app_id, &app_proposals)?;

    // add the first yes vote from voter
    let ballot = Ballot {
        weight: voting_power.amount.u128(),
        vote: Vote::Yes,
    };

    BALLOTS.save(deps.storage, (id, &info.sender), &ballot)?;
    VOTERDEPOSIT.save(deps.storage, (id, &info.sender), &info.funds)?;

    let propbyapp = match PROPOSALSBYAPP.may_load(deps.storage, propose.app_id)? {
        Some(data) => Some(data),
        None => Some(vec![]),
    };

    let mut app_gov_info = match APPGOVCONFIG.may_load(deps.storage, propose.app_id)? {
        Some(data) => data,
        None => AppGovConfig {
            proposal_count: 0,
            current_supply: Uint128::from(total_weight).u128(),
            active_participation_supply: 0,
        },
    };

    //// update proposal count
    app_gov_info.proposal_count += 1;
    //// update current supply
    app_gov_info.current_supply = Uint128::from(total_weight).u128();

    let mut proposals_by_app = propbyapp.unwrap();

    proposals_by_app.push(id);
    PROPOSALSBYAPP.save(deps.storage, propose.app_id, &proposals_by_app)?;
    APPGOVCONFIG.save(deps.storage, propose.app_id, &app_gov_info)?;

    Ok(Response::new()
        .add_attribute("action", "propose")
        .add_attribute("proposer", info.sender)
        .add_attribute("proposal_id", id.to_string())
        .add_attribute("status", format!("{:?}", prop.status)))
}

pub fn execute_vote(
    deps: DepsMut<ComdexQuery>,
    env: Env,
    info: MessageInfo,
    proposal_id: u64,
    vote: Vote,
) -> Result<Response<ComdexMessages>, ContractError> {
    // ensure proposal exists and can be voted on
    let mut prop = PROPOSALS.load(deps.storage, proposal_id)?;
    let status = prop.current_status(&env.block);

    // only Open voting status is eligible for voting
    if status != Status::Open {
        return Err(ContractError::NotOpen {});
    }

    //Get Proposal Start Height
    //Checking voting power 1 block prior to block height when proposal was raised
    let vote_power_height = prop.start_height - 1;

    let cfg = CONFIG.load(deps.storage)?;
    let token_denom = &prop.token_denom;

    //Get Voter power at proposal height -1
    let voting_power = query_owner_token_at_height(
        deps.as_ref(),
        info.sender.to_string(),
        token_denom.to_string(),
        vote_power_height.to_string(),
        cfg.target,
    )?;

    //check previous vote (if any) in order to change previous vote weights
    let previous_vote = BALLOTS.may_load(deps.storage, (proposal_id, &info.sender))?;

    if let Some(..) = previous_vote {
        let prev_vote = previous_vote.unwrap();
        prop.votes
            .subtract_vote(prev_vote.vote, voting_power.amount.u128())
    }

    let ballot_new = Ballot {
        weight: voting_power.amount.u128(),
        vote,
    };
    //update ballot vote
    BALLOTS.save(deps.storage, (proposal_id, &info.sender), &ballot_new)?;

    // update vote tally
    prop.votes.add_vote(vote, voting_power.amount.u128());
    prop.update_status(&env.block);
    PROPOSALS.save(deps.storage, proposal_id, &prop)?;

    Ok(Response::new()
        .add_attribute("action", "vote")
        .add_attribute("voter", info.sender)
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("status", format!("{:?}", prop.status))
        .add_attribute("vote", format!("{:?}", vote)))
}

pub fn execute_execute(
    deps: DepsMut<ComdexQuery>,
    env: Env,
    info: MessageInfo,
    proposal_id: u64,
) -> Result<Response<ComdexMessages>, ContractError> {
    //Anyone can trigger the execution if the proposal current status is Passed
    let mut prop = PROPOSALS.load(deps.storage, proposal_id)?;
    let status = prop.current_status(&env.block);

    if status != Status::Passed {
        return Err(ContractError::WrongExecuteStatus {});
    }

    //Set it to executed
    prop.status = Status::Executed;
    PROPOSALS.save(deps.storage, proposal_id, &prop)?;

    //Dispatch all proposed messages
    Ok(Response::new()
        .add_messages(prop.msgs)
        .add_attribute("action", "execute")
        .add_attribute("sender", info.sender)
        .add_attribute("proposal_id", proposal_id.to_string()))
}

pub fn execute_deposit(
    deps: DepsMut<ComdexQuery>,
    env: Env,
    info: MessageInfo,
    proposal_id: u64,
) -> Result<Response<ComdexMessages>, ContractError> {
    // Get proposal latest status
    let mut prop = PROPOSALS.load(deps.storage, proposal_id)?;
    let status = prop.current_status(&env.block);

    // only Open or Pending Proposals are eligible for deposit

    if [Status::Executed, Status::Rejected, Status::Passed]
        .iter()
        .any(|x| *x == status)
    {
        return Err(ContractError::CannotDeposit {});
    }

    // Get user deposit info for the proposal and update the deposit data

    let mut deposit_info = match VOTERDEPOSIT.may_load(deps.storage, (proposal_id, &info.sender))? {
        Some(record) => record,
        None => vec![],
    };
    let mut deposit_amount: u128 = 0;
    if deposit_info != vec![] {
        for mut current_deposit_coin in deposit_info.clone() {
            for new_deposit_coin in info.funds.clone() {
                if new_deposit_coin.denom == current_deposit_coin.denom {
                    current_deposit_coin.amount += new_deposit_coin.amount;
                    deposit_amount = new_deposit_coin.amount.u128();
                }
            }
        }
    } else {
        let mut is_correct_fund = false;
        for deposit_iter in info.funds.clone() {
            if prop.token_denom == deposit_iter.denom {
                deposit_amount = deposit_iter.amount.u128();
                is_correct_fund = true;
            }
        }
        if is_correct_fund {
            deposit_info = info.funds;
        } else {
            return Err(ContractError::IncorrectDenomDeposit {});
        }
    }

    prop.current_deposit += deposit_amount;

    if Uint128::from(prop.current_deposit) > Uint128::from(prop.min_deposit) {
        prop.status = Status::Open
    }

    VOTERDEPOSIT.save(deps.storage, (proposal_id, &info.sender), &deposit_info)?;
    PROPOSALS.save(deps.storage, proposal_id, &prop)?;

    Ok(Response::new()
        .add_attribute("action", "deposit")
        .add_attribute("depositor", info.sender)
        .add_attribute("proposal_id", proposal_id.to_string()))
}

pub fn execute_refund(
    deps: DepsMut<ComdexQuery>,
    env: Env,
    info: MessageInfo,
    proposal_id: u64,
) -> Result<Response<ComdexMessages>, ContractError> {
    // Get proposal status
    let prop = PROPOSALS.load(deps.storage, proposal_id)?;
    let status = prop.current_status(&env.block);

    // Open and Pending proposal status are not eligible for refund
    if [Status::Pending, Status::Open].iter().any(|x| *x == status) {
        return Err(ContractError::NonPassedProposalRefund {});
    }

    //disallow slashed proposal
    if status == Status::Rejected && prop.check_vetoed(&env.block) {
        return Err(ContractError::SlashedProposal {});
    }

    // get sender deposit info
    let deposit_info = VOTERDEPOSIT.may_load(deps.storage, (proposal_id, &info.sender))?;

    // If no reposit for the proposal
    if deposit_info.is_none() {
        return Err(ContractError::NoDeposit {});
    }

    //// need to update current_deposit////////

    VOTERDEPOSIT.remove(deps.storage, (proposal_id, &info.sender));

    PROPOSALS.save(deps.storage, proposal_id, &prop)?;

    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: deposit_info.unwrap(),
        })
        .add_attribute("action", "refund")
        .add_attribute("sender", info.sender)
        .add_attribute("proposal_id", proposal_id.to_string()))
}

pub fn execute_slash(
    deps: DepsMut<ComdexQuery>,
    env: Env,
    info: MessageInfo,
    proposal_id: u64,
) -> Result<Response<ComdexMessages>, ContractError> {
    let mut prop = PROPOSALS.load(deps.storage, proposal_id)?;
    let status = prop.current_status(&env.block);

    //Check if proposal is rejected
    if status != Status::Rejected {
        return Err(ContractError::NotRejected {});
    }

    // check if proposal is vetoed
    if !prop.check_vetoed(&env.block) {
        return Err(ContractError::ProposalNotVetoed {});
    }

    //check if proposal already slashed
    if prop.is_slashed {
        return Err(ContractError::AlreadySlashed {});
    }

    let deposit_amount = prop.current_deposit;
    let deposit_denom = prop.token_denom.clone();
    let slash_amount = Coin {
        amount: Uint128::from(deposit_amount),
        denom: deposit_denom,
    };
    prop.is_slashed = true;

    PROPOSALS.save(deps.storage, proposal_id, &prop)?;

    Ok(Response::new()
        .add_message(ComdexMessages::MsgBurnGovTokensForApp {
            app_mapping_id: prop.app_mapping_id,
            amount: slash_amount,
            from: env.contract.address.to_string(),
        })
        .add_attribute("action", "Slash")
        .add_attribute("trigger_address", info.sender)
        .add_attribute("proposal_id", proposal_id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ComdexQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Threshold { proposal_id } => to_binary(&query_threshold(deps, proposal_id)?),
        QueryMsg::Proposal { proposal_id } => {
            to_binary(&query_proposal_detailed(deps, env, proposal_id)?)
        }
        QueryMsg::Vote { proposal_id, voter } => to_binary(&query_vote(deps, proposal_id, voter)?),
        QueryMsg::ListProposals { start_after, limit } => {
            to_binary(&list_proposals(deps, env, start_after, limit)?)
        }
        QueryMsg::ReverseProposals {
            start_before,
            limit,
        } => to_binary(&reverse_proposals(deps, env, start_before, limit)?),
        QueryMsg::ListVotes {
            proposal_id,
            start_after,
            limit,
        } => to_binary(&list_votes(deps, proposal_id, start_after, limit)?),
        QueryMsg::ListAppProposal { app_id } => {
            to_binary(&get_proposals_by_app(deps, env, app_id)?)
        }
        QueryMsg::AppAllUpData { app_id } => to_binary(&get_all_up_info_by_app(deps, env, app_id)?),
    }
}

fn query_threshold(deps: Deps<ComdexQuery>, proposal_id: u64) -> StdResult<ThresholdResponse> {
    let cfg = CONFIG.load(deps.storage)?;
    let prop = PROPOSALS.load(deps.storage, proposal_id)?;

    Ok(cfg.threshold.to_response(prop.total_weight))
}

fn query_proposal_detailed(
    deps: Deps<ComdexQuery>,
    env: Env,
    id: u64,
) -> StdResult<ProposalResponseTotal> {
    let prop = PROPOSALS.load(deps.storage, id)?;
    let status = prop.current_status(&env.block);
    Ok(ProposalResponseTotal {
        id,
        title: prop.title,
        description: prop.description,
        msgs: prop.msgs,
        status,
        start_time: prop.start_time,
        expires: prop.expires,
        votes: prop.votes,
        duration: prop.duration,
        start_height: prop.start_height,
        threshold: prop.threshold,
        proposer: prop.proposer,
        token_denom: prop.token_denom,
        total_weight: prop.total_weight,
        current_deposit: prop.current_deposit,
    })
}

// settings for pagination
const MAX_LIMIT: u32 = 300;
const DEFAULT_LIMIT: u32 = 10;

fn list_proposals(
    deps: Deps<ComdexQuery>,
    env: Env,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<ProposalListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let proposals = PROPOSALS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|p| map_proposal(&env.block, p))
        .collect::<StdResult<_>>()?;

    Ok(ProposalListResponse { proposals })
}

fn get_proposals_by_app(
    deps: Deps<ComdexQuery>,
    env: Env,
    app_id: u64,
) -> StdResult<Vec<ProposalResponseTotal>> {
    let info = match PROPOSALSBYAPP.may_load(deps.storage, app_id)? {
        Some(record) => record,
        None => vec![],
    };

    let mut all_proposals = vec![];

    for i in info {
        let proposal = query_proposal_detailed(deps, env.clone(), i)?;
        all_proposals.push(proposal);
    }

    Ok(all_proposals)
}

fn get_all_up_info_by_app(
    deps: Deps<ComdexQuery>,
    env: Env,
    app_id: u64,
) -> StdResult<AppGovConfig> {
    let info = match PROPOSALSBYAPP.may_load(deps.storage, app_id)? {
        Some(record) => record,
        None => vec![],
    };

    let app_response = query_app_exists(deps, app_id)?;
    let gov_token_id = app_response.gov_token_id;
    let total_weight = get_token_supply(deps, app_id, gov_token_id)?;

    let mut participation_info = APPGOVCONFIG.may_load(deps.storage, app_id)?.unwrap();
    let mut total_votes_weight: u128 = 0;
    for i in info {
        let proposal = query_proposal_detailed(deps, env.clone(), i)?;
        total_votes_weight += proposal.votes.total();
    }
    participation_info.current_supply = Uint128::from(total_weight).u128();
    participation_info.active_participation_supply = total_votes_weight;

    Ok(participation_info)
}

fn reverse_proposals(
    deps: Deps<ComdexQuery>,
    env: Env,
    start_before: Option<u64>,
    limit: Option<u32>,
) -> StdResult<ProposalListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let end = start_before.map(Bound::exclusive);
    let props: StdResult<Vec<_>> = PROPOSALS
        .range(deps.storage, None, end, Order::Descending)
        .take(limit)
        .map(|p| map_proposal(&env.block, p))
        .collect();

    Ok(ProposalListResponse { proposals: props? })
}

fn map_proposal(
    block: &BlockInfo,
    item: StdResult<(u64, Proposal)>,
) -> StdResult<ProposalResponse> {
    item.map(|(id, prop)| {
        let status = prop.current_status(block);
        let threshold = prop.threshold.to_response(prop.total_weight);
        ProposalResponse {
            id,
            title: prop.title,
            description: prop.description,
            msgs: prop.msgs,
            status,
            expires: prop.expires,
            threshold,
        }
    })
}

fn query_vote(deps: Deps<ComdexQuery>, proposal_id: u64, voter: String) -> StdResult<VoteResponse> {
    let voter = deps.api.addr_validate(&voter)?;
    let ballot = BALLOTS.may_load(deps.storage, (proposal_id, &voter))?;
    let vote = ballot.map(|b| VoteInfo {
        proposal_id,
        voter: voter.into(),
        vote: b.vote,
        weight: b.weight,
    });
    Ok(VoteResponse { vote })
}

fn list_votes(
    deps: Deps<ComdexQuery>,
    proposal_id: u64,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<VoteListResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let votes = BALLOTS
        .prefix(proposal_id)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            item.map(|(addr, ballot)| VoteInfo {
                proposal_id,
                voter: addr.into(),
                vote: ballot.vote,
                weight: ballot.weight,
            })
        })
        .collect::<StdResult<_>>()?;

    Ok(VoteListResponse { votes })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{Addr, OwnedDeps};
    use cosmwasm_std::{BankMsg, Decimal};
    use cw_storage_plus::Map;
    use cw_utils::Expiration;
    use cw_utils::{Duration, Threshold};
    use std::marker::PhantomData;

    use super::*;

    const OWNER: &str = "admin0001";

    pub fn mock_dependencies1() -> OwnedDeps<MockStorage, MockApi, MockQuerier, ComdexQuery> {
        OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: MockQuerier::default(),
            custom_query_type: PhantomData,
        }
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies1();
        let info = mock_info(OWNER, &[]);

        let instantiate_msg = InstantiateMsg {
            threshold: Threshold::ThresholdQuorum {
                threshold: Decimal::percent(50),
                quorum: Decimal::percent(33),
            },
            target: "0.0.0.0090".to_string(),
        };

        let err = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg);

        assert_ne!(err, Err(ContractError::AbsoluteCountNotAccepted {}));
        assert_ne!(err, Err(ContractError::AbsolutePercentageNotAccepted {}));
    }
    // Propose Testcase
    #[test]
    fn test_propose() {
        let mut deps = mock_dependencies1();
        //let mut deps2=mock_dependencies1();
        let info = mock_info(OWNER, &[]);
        let msgs_com = vec![
            ComdexMessages::MsgWhitelistAppIdVaultInterest { app_mapping_id: 33 },
            ComdexMessages::MsgWhitelistAppIdVaultInterest { app_mapping_id: 34 },
        ];

        let propose_1 = Propose {
            title: "propose".to_string(),
            description: "test_propose".to_string(),
            msgs: msgs_com,
            // note: we ignore API-spec'd earliest if passed, always opens immediately
            latest: Some(Expiration::Never {}),
            app_id: 33,
        };

        //let msgs_length=msgs_com.len();
        let k = execute_propose(deps.as_mut(), mock_env(), info.clone(), propose_1);
        assert_eq!(k, Err(ContractError::ExtraMessages {}));
        let msgs_2: Vec<ComdexMessages> = vec![];
        let propose_2 = Propose {
            title: "propose".to_string(),
            description: "test_propose".to_string(),
            msgs: msgs_2,
            // note: we ignore API-spec'd earliest if passed, always opens immediately
            latest: Some(Expiration::Never {}),
            app_id: 33,
        };
        let f = execute_propose(deps.as_mut(), mock_env(), info, propose_2);
        assert_eq!(f, Err(ContractError::NoMessage {}));
    }

    //Execute Testcase
    #[test]
    fn test_execute() {
        let mut deps = mock_dependencies1();
        let info = mock_info(OWNER, &[]);
        let ts = cosmwasm_std::Timestamp::from_nanos(1_655_745_339);
        let a = Uint128::from(123u128);
        pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
        let id = next_id(&mut deps.storage).unwrap();
        let mut prop = Proposal {
            title: "prop".to_string(),
            start_time: ts,
            description: "test prop".to_string(),
            start_height: 43,
            expires: Expiration::AtTime(cosmwasm_std::Timestamp::from_nanos(1_655_745_430)),
            msgs: vec![ComdexMessages::MsgWhitelistAppIdVaultInterest { app_mapping_id: 33 }],
            status: Status::Passed,
            duration: Duration::Time(40),
            threshold: Threshold::ThresholdQuorum {
                threshold: Decimal::percent(50),
                quorum: Decimal::percent(33),
            },
            total_weight: 14,
            votes: Votes {
                yes: 32,
                no: 24,
                abstain: 10,
                veto: 3,
            },
            deposit: vec![Coin {
                denom: "vote here".to_string(),
                amount: a,
            }],
            proposer: "validator201".to_string(),
            token_denom: "toVote".to_string(),
            min_deposit: 45,
            current_deposit: 56,
            app_mapping_id: id,
            is_slashed: true,
        };

        prop.update_status(&mock_env().block);

        let _k = PROPOSALS.save(&mut deps.storage, id, &prop);

        let err = execute_execute(deps.as_mut(), mock_env(), info, id);
        assert_ne!(err, Err(ContractError::WrongExecuteStatus {}));
        assert_ne!(err, Err(ContractError::NotExpiredYet {}));

        assert_eq!(
            err,
            Ok(Response::new()
                .add_messages(prop.msgs)
                .add_attribute("action", "execute")
                .add_attribute("sender", OWNER)
                .add_attribute("proposal_id", id.to_string()))
        );
    }

    //    Refund Testcase
    #[test]
    fn test_refund_works() {
        // let id = next_id(&mut deps.storage).unwrap();
        let mut deps = mock_dependencies1();
        let deposit_amount = Uint128::from(10u128);
        let info = mock_info(
            OWNER,
            &[Coin {
                denom: "coin".to_string(),
                amount: deposit_amount,
            }],
        );
        let _v1 = Vote::Yes;
        let ts = cosmwasm_std::Timestamp::from_nanos(1_655_794_117);
        let id = next_id(&mut deps.storage).unwrap();
        pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
        let prop = Proposal {
            title: "prop".to_string(),
            start_time: ts,
            description: "test prop".to_string(),
            start_height: 43,
            expires: Expiration::Never {},
            msgs: vec![ComdexMessages::MsgWhitelistAppIdVaultInterest { app_mapping_id: 33 }],
            status: Status::Pending,
            duration: Duration::Time(50000000),
            threshold: Threshold::ThresholdQuorum {
                threshold: Decimal::percent(50),
                quorum: Decimal::percent(33),
            },
            total_weight: 14,
            votes: Votes {
                yes: 10,
                no: 5,
                abstain: 10,
                veto: 39,
            },
            deposit: vec![Coin {
                denom: "vote here".to_string(),
                amount: deposit_amount,
            }],
            proposer: "validator201".to_string(),
            token_denom: "toVote".to_string(),
            min_deposit: 33,
            current_deposit: 56,
            app_mapping_id: id,
            is_slashed: false,
        };
        // if status is pending should get non passedProposalRefund error
        let mut _k = PROPOSALS.save(&mut deps.storage, id, &prop);
        let mut prop = PROPOSALS.load(&deps.storage, id).unwrap();
        assert_eq!(prop.status, Status::Pending);
        let g = execute_refund(deps.as_mut(), mock_env(), info.clone(), id);
        assert_eq!(g, Err(ContractError::NonPassedProposalRefund {}));

        // If status is Rejected Should get Slashedpropsal Error
        prop.status = Status::Rejected;
        _k = PROPOSALS.save(&mut deps.storage, id, &prop);
        let z = execute_refund(deps.as_mut(), mock_env(), info.clone(), id);
        assert_eq!(z, Err(ContractError::SlashedProposal {}));

        prop.status = Status::Passed;
        _k = PROPOSALS.save(&mut deps.storage, id, &prop);
        let mut prop = PROPOSALS.load(&deps.storage, id).unwrap();
        assert_eq!(prop.status, Status::Passed);
        let votes = prop.votes.clone();
        assert_eq!(39, votes.veto);

        prop.status = Status::Rejected;
        prop.expires = Expiration::AtTime(cosmwasm_std::Timestamp::from_nanos(1_655_794_157));
        let mut _prop = PROPOSALS.save(&mut deps.storage, id, &prop);
        let mut prop = PROPOSALS.load(&deps.storage, id).unwrap();
        assert_eq!(prop.status, Status::Rejected);
        let i = execute_refund(deps.as_mut(), mock_env(), info.clone(), id);
        assert_eq!(i, Err(ContractError::SlashedProposal {}));
        assert_eq!(
            (Decimal::percent(33) * Uint128::from(votes.total())).u128(),
            21
        );
        let _votes = prop.votes.clone();

        prop.status = Status::Passed;
        prop.expires = Expiration::Never {};
        _prop = PROPOSALS.save(&mut deps.storage, id, &prop);
        pub const VOTERDEPOSIT: Map<(u64, &Addr), Vec<Coin>> = Map::new("voter deposit");
        let deposit_info = VOTERDEPOSIT
            .may_load(&deps.storage, (id, &info.sender))
            .unwrap();
        assert_eq!(deposit_info, None);
        let j = execute_refund(deps.as_mut(), mock_env(), info.clone(), id);
        assert_eq!(j, Err(ContractError::NoDeposit {}));

        prop.status = Status::Open;
        let a = Uint128::from(123u128);
        let deposit_info1 = Some(vec![Coin {
            denom: "coin".to_string(),
            amount: a,
        }])
        .unwrap();
        let mut _vot = VOTERDEPOSIT.save(&mut deps.storage, (id, &info.sender), &deposit_info1);
        _vot = PROPOSALS.save(&mut deps.storage, id, &prop);
        prop.status = Status::Passed;
        _prop = PROPOSALS.save(&mut deps.storage, id, &prop);
        let k = execute_refund(deps.as_mut(), mock_env(), info.clone(), id);
        assert_eq!(
            k,
            Ok(Response::new()
                .add_message(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: deposit_info1,
                })
                .add_attribute("action", "refund")
                .add_attribute("sender", info.sender)
                .add_attribute("proposal_id", id.to_string()))
        );
    }

    //   Deposit Testcase
    #[test]
    fn test_deposit() {
        let mut deps = mock_dependencies1();
        let a = Uint128::from(123u128);
        let info = mock_info(
            OWNER,
            &[Coin {
                denom: "coin".to_string(),
                amount: a,
            }],
        );
        let ts = cosmwasm_std::Timestamp::from_nanos(1_655_745_339);
        let a = Uint128::from(123u128);
        pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
        let id = next_id(&mut deps.storage).unwrap();

        let mut prop = Proposal {
            title: "prop".to_string(),
            start_time: ts,
            description: "test prop".to_string(),
            start_height: 43,
            expires: Expiration::AtTime(cosmwasm_std::Timestamp::from_nanos(1_655_745_430)),
            msgs: vec![ComdexMessages::MsgWhitelistAppIdVaultInterest { app_mapping_id: 33 }],
            status: Status::Executed,
            duration: Duration::Time(40),
            threshold: Threshold::ThresholdQuorum {
                threshold: Decimal::percent(50),
                quorum: Decimal::percent(33),
            },
            total_weight: 14,
            votes: Votes {
                yes: 10,
                no: 5,
                abstain: 10,
                veto: 39,
            },
            deposit: vec![Coin {
                denom: "vote here".to_string(),
                amount: a,
            }],
            proposer: "validator201".to_string(),
            token_denom: "toVote".to_string(),
            min_deposit: 45,
            current_deposit: 56,
            app_mapping_id: id,
            is_slashed: true,
        };

        prop.update_status(&mock_env().block);
        let mut _prop = PROPOSALS.save(&mut deps.storage, id, &prop);
        let mut _vote = VOTERDEPOSIT.save(&mut deps.storage, (id, &info.sender), &info.funds);
        let _deposit_info = VOTERDEPOSIT
            .may_load(&deps.storage, (id, &info.sender))
            .unwrap();
        let err = execute_deposit(deps.as_mut(), mock_env(), info.clone(), id);
        assert_eq!(err, Err(ContractError::CannotDeposit {}));
        prop.status = Status::Open;
        _prop = PROPOSALS.save(&mut deps.storage, id, &prop);
        let a = Uint128::from(123u128);
        pub const VOTERDEPOSIT: Map<(u64, &Addr), Vec<Coin>> = Map::new("voter deposit");
        let deposit_info1 = Some(vec![Coin {
            denom: "coin".to_string(),
            amount: a,
        }])
        .unwrap();
        let mut _deposit = VOTERDEPOSIT.save(&mut deps.storage, (id, &info.sender), &deposit_info1);

        prop.status = Status::Open;
        prop.expires = Expiration::Never {};
        _prop = PROPOSALS.save(&mut deps.storage, id, &prop);
        //  If the status is not equal to open or pending, the error "CannotDeposit" will appear.
        let err = execute_deposit(deps.as_mut(), mock_env(), info.clone(), id);
        assert_ne!(err, Err(ContractError::CannotDeposit {}));
        assert_ne!(err, Err(ContractError::IncorrectDenomDeposit {}));

        let z = execute_deposit(deps.as_mut(), mock_env(), info.clone(), id);
        assert_eq!(
            z,
            Ok(Response::new()
                .add_attribute("action", "deposit")
                .add_attribute("depositor", info.sender)
                .add_attribute("proposal_id", id.to_string()))
        );
    }

    //Vote Testcase
    #[test]
    fn test_vote() {
        let mut deps = mock_dependencies1();
        let a = Uint128::from(123u128);
        let info = mock_info(OWNER, &[]);
        let ts = cosmwasm_std::Timestamp::from_nanos(1_655_827_190);
        pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
        let id = next_id(&mut deps.storage).unwrap();
        let mut prop = Proposal {
            title: "prop".to_string(),
            start_time: ts,
            description: "test prop".to_string(),
            start_height: 43,
            expires: Expiration::AtTime(cosmwasm_std::Timestamp::from_nanos(1_655_897_190)),
            msgs: vec![ComdexMessages::MsgWhitelistAppIdVaultInterest { app_mapping_id: id }],
            status: Status::Passed,
            duration: Duration::Time(400000000),
            threshold: Threshold::ThresholdQuorum {
                threshold: Decimal::percent(50),
                quorum: Decimal::percent(33),
            },
            total_weight: 40,
            votes: Votes {
                yes: 32,
                no: 24,
                abstain: 10,
                veto: 3,
            },
            deposit: vec![Coin {
                denom: "vote here".to_string(),
                amount: a,
            }],
            proposer: "validator201".to_string(),
            token_denom: "toVote".to_string(),
            min_deposit: 45,
            current_deposit: 56,
            app_mapping_id: id,
            is_slashed: false,
        };

        let mut _prop = PROPOSALS.save(&mut deps.storage, id, &prop);

        let prop1 = PROPOSALS.load(&deps.storage, id).unwrap();
        assert_eq!(prop1.current_status(&mock_env().block), Status::Passed);

        // If the status is not equal to "open," an error message will appear.
        let k = execute_vote(deps.as_mut(), mock_env(), info.clone(), id, Vote::Yes);
        assert_eq!(k, Err(ContractError::NotOpen {}));
        prop.status = Status::Open;
        prop.expires = Expiration::Never {};
        _prop = PROPOSALS.save(&mut deps.storage, id, &prop);
        let prop1 = PROPOSALS.load(&deps.storage, id).unwrap();
        assert_eq!(prop1.expires, Expiration::Never {});
        let _m = execute_vote(deps.as_mut(), mock_env(), info, id, Vote::Yes);
        assert_eq!(prop1.status, Status::Open);
        assert_eq!(prop1.current_status(&mock_env().block), Status::Open);
        assert!(!prop.expires.is_expired(&mock_env().block));
        assert_eq!(prop1.current_status(&mock_env().block), Status::Open);
        assert_eq!(prop.current_status(&mock_env().block), Status::Open);
        assert_eq!(prop.status, Status::Open);
    }
}
