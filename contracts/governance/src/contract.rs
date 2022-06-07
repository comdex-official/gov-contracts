use std::cmp::Ordering;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, BlockInfo, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdResult,BankMsg,Coin, Uint128,
};
use crate::coin_helpers::{ assert_sent_sufficient_coin_deposit};
use comdex_bindings::ComdexMessages;
use comdex_bindings::{ComdexQuery};
use cw2::set_contract_version;
use cw3::{
    ProposalListResponse, ProposalResponse, Status, Vote, VoteInfo, VoteListResponse, VoteResponse,
    };
use cw_storage_plus::Bound;
use cw_utils::{Expiration, ThresholdResponse,Duration,Threshold};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg,ProposalResponseTotal};
use crate::state::{next_id, Ballot, Config, Proposal, Votes,AppGovConfig, BALLOTS, CONFIG, PROPOSALS,PROPOSALSBYAPP,VOTERDEPOSIT,APPPROPOSALS,APPGOVCONFIG};
use crate::validation::{whitelistassetlockereligible,query_owner_token_at_height,query_app_exists,get_token_supply,query_get_asset_data,
    whitelistassetlockerrewards,whitelistappidvaultinterest,validate_threshold,addextendedpairvault,collectorlookuptable,updatepairvaultstability
,auctionmappingforapp,updatelockerlsr,removewhitelistassetlocker,removewhitelistappidvaultinterest,whitelistappidliquidation,removewhitelistappidliquidation};


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

    match msg.threshold {
        Threshold::AbsoluteCount{weight:_}=> return Err(ContractError::AbsoluteCountNotAccepted {}),
        Threshold::AbsolutePercentage{percentage:_}=> return Err(ContractError::AbsolutePercentageNotAccepted {}),
        Threshold::ThresholdQuorum{threshold,quorum}=>validate_threshold(&threshold,&quorum)?

    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let cfg = Config {
        threshold: msg.threshold,
        target:msg.target,
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
        ExecuteMsg::Propose {
            title,
            description,
            msgs,
            latest,
            app_id,
        } => execute_propose(deps, env, info, title, description, msgs, app_id,latest),
        ExecuteMsg::Vote { proposal_id, vote } => execute_vote(deps, env, info, proposal_id, vote),
        ExecuteMsg::Execute { proposal_id } => execute_execute(deps, env, info, proposal_id),
        ExecuteMsg::Refund { proposal_id } => execute_refund(deps, env, info, proposal_id),
        ExecuteMsg::Deposit { proposal_id} => execute_deposit(deps, env, info, proposal_id),

    }
}


pub fn execute_propose(
    deps: DepsMut<ComdexQuery>,
    env: Env,
    info: MessageInfo,
    title: String,
    description: String,
    msgs: Vec<ComdexMessages>,
    app_id :u64,
    latest: Option<Expiration>,
) -> Result<Response<ComdexMessages>, ContractError> {
    
    // get proposal message length
    let msg_length=msgs.len();
    
    // should be only 1 message
    if msg_length>1{
        return Err(ContractError::ExtraMessages {})
    }
    
    //throw empty message error
    if msgs.is_empty() {
        return Err(ContractError::NoMessage {});
    }
    
    //Get App Data for app_id 
    let app_response=query_app_exists(deps.as_ref(),  app_id)?;
    //if app response exists gov data
    let voting_time=app_response.gov_time_in_seconds;
    let min_gov_deposit=app_response.min_gov_deposit;
    let max_voting_period =Duration::Time(voting_time);
    let gov_token_id=app_response.gov_token_id;
    
    //Get gov token denom name
    let gov_token_denom=query_get_asset_data(deps.as_ref(),gov_token_id)?;
    if gov_token_denom=="" || gov_token_id==0 {
        return Err(ContractError::NoGovToken {});
    }

    //Get Total Supply for denom to get proposal weight
    let total_weight=get_token_supply(deps.as_ref(),app_id,gov_token_id)?;
    if total_weight ==0 
    {
        return Err(ContractError::ZeroSupply {});
    }
    
    let min_deposit=Coin{amount:Uint128::from(min_gov_deposit), denom:gov_token_denom.clone()};
    let cfg = CONFIG.load(deps.storage)?;
    let height=env.block.height-1;

    //Calculate Proposer voting Power

    let voting_power=query_owner_token_at_height(deps.as_ref(),info.sender.to_string(),gov_token_denom.to_string(),height.to_string(),cfg.target)?;

    // max expires also used as default
    let max_expires = max_voting_period.after(&env.block);
    let mut expires = latest.unwrap_or(max_expires);
    let comp = expires.partial_cmp(&max_expires);
    if let Some(Ordering::Greater) = comp {
        expires = max_expires;
    } else if comp.is_none() {
        return Err(ContractError::WrongExpiration {});
    }

    //Check if deposited is more than Min-Deposit


    //Check if no other deposit provided other than gov token deposit
    let funds_len=info.funds.len();
    
    if funds_len>1{
        return Err(ContractError::AdditionalDenomDeposit {})
    }
    //check if gov denom exists in user deposit
    
    //check current deposit
    
    let mut current_deposit:u128=0;
    let mut is_correct_denom=false;
    for user_deposit in &info.funds{
        if user_deposit.denom.eq(&gov_token_denom){
            is_correct_denom=true;
            current_deposit=user_deposit.amount.u128();

        }
    }

    if !is_correct_denom{
        return Err(ContractError::DenomNotFound {})
    }

    for msg in msgs.clone()
    {
        match msg{
            ComdexMessages::MsgWhiteListAssetLocker{app_mapping_id,asset_id}=>whitelistassetlockereligible(deps.as_ref(),app_mapping_id,asset_id,app_id)?,
            ComdexMessages::MsgWhitelistAppIdLockerRewards{app_mapping_id,asset_id}=>whitelistassetlockerrewards(deps.as_ref(),app_mapping_id,asset_id,app_id)?,
            ComdexMessages::MsgWhitelistAppIdVaultInterest{app_mapping_id}=>whitelistappidvaultinterest(deps.as_ref(),app_mapping_id,app_id)?,
            ComdexMessages::MsgAddExtendedPairsVault{app_mapping_id,
                                                    pair_id,
                                                    liquidation_ratio:_,
                                                    stability_fee,
                                                    closing_fee,
                                                    liquidation_penalty:_,
                                                    draw_down_fee,
                                                    is_vault_active:_,
                                                    debt_ceiling,
                                                    debt_floor,
                                                    is_psm_pair:_,
                                                    min_cr:_,
                                                    pair_name,
                                                    asset_out_oracle_price:_,
                                                    asset_out_price:_,
                                                    min_usd_value_left:_} =>addextendedpairvault(deps.as_ref(),app_mapping_id,pair_id,stability_fee,closing_fee,
                                                                                               draw_down_fee,debt_ceiling,debt_floor,pair_name,app_id)?,
            ComdexMessages::MsgSetCollectorLookupTable{ app_mapping_id ,
                                                        collector_asset_id ,
                                                        secondary_asset_id ,
                                                        surplus_threshold:_ ,
                                                        debt_threshold:_,
                                                        locker_saving_rate:_,
                                                        lot_size:_ ,
                                                        bid_factor:_} =>collectorlookuptable(deps.as_ref(),app_mapping_id,collector_asset_id,secondary_asset_id,app_id)?,

            ComdexMessages::MsgUpdateLsrInPairsVault{app_mapping_id,
                                                     ext_pair_id,
                                                     liquidation_ratio:_,
                                                     stability_fee:_,
                                                     closing_fee:_,
                                                     liquidation_penalty:_,
                                                     draw_down_fee:_,
                                                     min_cr:_,
                                                     debt_ceiling:_,
                                                     debt_floor:_,
                                                     min_usd_value_left:_}=>updatepairvaultstability(deps.as_ref(),app_mapping_id,ext_pair_id,app_id)?,


            ComdexMessages::MsgSetAuctionMappingForApp{app_mapping_id,
                                                       asset_id:_,
                                                       is_surplus_auction:_,
                                                       is_debt_auction:_,
                                                        asset_out_oracle_price:_,
                                                        asset_out_price:_} =>auctionmappingforapp(deps.as_ref(),app_mapping_id,app_id)?,

            ComdexMessages::MsgUpdateLsrInCollectorLookupTable{app_mapping_id,asset_id,lsr:_}=>updatelockerlsr(deps.as_ref(),app_mapping_id,asset_id,app_id)?,
            ComdexMessages::MsgRemoveWhitelistAssetLocker{app_mapping_id,asset_id}=> removewhitelistassetlocker(deps.as_ref(),app_mapping_id,asset_id,app_id)?,   
            ComdexMessages::MsgRemoveWhitelistAppIdVaultInterest{app_mapping_id}=>removewhitelistappidvaultinterest(deps.as_ref(),app_mapping_id,app_id)?,
            ComdexMessages::MsgWhitelistAppIdLiquidation{app_mapping_id}=>whitelistappidliquidation(deps.as_ref(),app_mapping_id,app_id)?,
            ComdexMessages::MsgRemoveWhitelistAppIdLiquidation{app_mapping_id}=>removewhitelistappidliquidation(deps.as_ref(),app_mapping_id,app_id)?,
        }
    }


    let deposit_status=assert_sent_sufficient_coin_deposit(&info.funds,  Some(min_deposit))?;
    // create a proposal
    let mut prop = Proposal {
        title,
        description,
        start_time:env.block.time,
        start_height: env.block.height,
        expires,
        msgs,
        duration:max_voting_period,
        status: deposit_status,
        votes: Votes::yes(voting_power.amount.u128()),
        threshold: cfg.threshold,
        total_weight: Uint128::from(total_weight).u128(),
        deposit:info.funds.clone(),
        proposer :info.sender.to_string(),
        token_denom : gov_token_denom.clone(),
        min_deposit:min_gov_deposit,
        current_deposit:current_deposit,
    };
    
    
 
    let mut app_proposals = match APPPROPOSALS.may_load(deps.storage, app_id)?
    {
        Some(record) => record,
        None => vec![]
    };
    

    prop.update_status(&env.block);
    
    //get latest proposal id counter
    let id = next_id(deps.storage)?;
    PROPOSALS.save(deps.storage, id, &prop)?;
    app_proposals.push(crate::state::AppProposalConfig { proposal_id: id, proposal: prop.clone() });
    APPPROPOSALS.save(deps.storage, app_id, &app_proposals)?;
    // add the first yes vote from voter
    let ballot = Ballot {
        weight: voting_power.amount.u128(),
        vote: Vote::Yes,
    };
    
    BALLOTS.save(deps.storage, (id, &info.sender), &ballot)?;
    VOTERDEPOSIT.save(deps.storage, (id, &info.sender), &info.funds)?;

    let  propbyapp = match PROPOSALSBYAPP.may_load(deps.storage, app_id)?
    {Some(data)=>Some(data),
        None=>Some(vec![])};

    let mut app_gov_info = match APPGOVCONFIG.may_load(deps.storage, app_id)?
    {Some(data)=>data,
        None=>(AppGovConfig{proposal_count:0,
                                current_supply:Uint128::from(total_weight).u128(),
                                active_participation_supply:0})};
    
    
    app_gov_info.proposal_count=app_gov_info.proposal_count+1;
    app_gov_info.current_supply= Uint128::from(total_weight).u128();      


    let mut app_proposals=propbyapp.unwrap();

    app_proposals.push(id);
    PROPOSALSBYAPP.save(deps.storage, app_id, &app_proposals)?;
    APPGOVCONFIG.save(deps.storage, app_id, &app_gov_info)?;

    Ok(Response::new()
        .add_attribute("action", "propose")
        .add_attribute("sender", info.sender)
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

    if status != Status::Open {
        return Err(ContractError::NotOpen {});
    }
    if prop.expires.is_expired(&env.block) {
        return Err(ContractError::Expired {});
    }

    //Get Proposal Start Height
    let vote_power_height=prop.start_height-1;

    let cfg = CONFIG.load(deps.storage)?;
    let token_denom=&prop.token_denom;
    
    //Get Voter power at proposal height 
    let voting_power=query_owner_token_at_height(deps.as_ref(),info.sender.to_string(),token_denom.to_string(),vote_power_height.to_string(),cfg.target)?;

    let previous_vote= BALLOTS.may_load(deps.storage, (proposal_id, &info.sender))?;

     if previous_vote.is_some()
     {
         let prev_vote=previous_vote.unwrap();
         prop.votes.subtract_vote(prev_vote.vote, voting_power.amount.u128())
     }

     let ballot_new=Ballot {
        weight: voting_power.amount.u128(),
        vote,
    };

    BALLOTS.save(deps.storage, (proposal_id, &info.sender), &ballot_new)?;
    
    // update vote tally
    prop.votes.add_vote(vote, voting_power.amount.u128());
    prop.update_status(&env.block);
    PROPOSALS.save(deps.storage, proposal_id, &prop)?;
    Ok(Response::new()
        .add_attribute("action", "vote")
        .add_attribute("sender", info.sender)
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("status", format!("{:?}", prop.status)))
}

pub fn execute_execute(
    deps: DepsMut<ComdexQuery>,
    env: Env,
    info: MessageInfo,
    proposal_id: u64,
) -> Result<Response<ComdexMessages>, ContractError> {
    // anyone can trigger the execution if the vote passed

    let mut prop = PROPOSALS.load(deps.storage, proposal_id)?;
    let status = prop.current_status(&env.block);

    // we allow execution even after the proposal "expiration" as long as all vote come in before
    // that point. If it was approved on time, it can be executed any time.
    if status != Status::Passed {
        return Err(ContractError::WrongExecuteStatus {});
    }

    //cannot be executed until voting period is expired
    if !prop.expires.is_expired(&env.block) {
        return Err(ContractError::NotExpiredYet {});
    }

    // set it to executed
    prop.status = Status::Executed;
    PROPOSALS.save(deps.storage, proposal_id, &prop)?;

    // dispatch all proposed messages
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

    let mut  prop = PROPOSALS.load(deps.storage, proposal_id)?;
    let status = prop.current_status(&env.block);

    // only Open or Pending Proposals are eligible for deposit

    if [ Status::Executed,Status::Rejected,Status::Passed]
    .iter()
    .any(|x| *x == status)
    {
    return  Err(ContractError::CannotDeposit {});
    }

    // Get user deposit info for the proposal and update the deposit data

    let mut deposit_info = match VOTERDEPOSIT.may_load(deps.storage, (proposal_id, &info.sender))?
    {
        Some(record) => record,
        None => vec![]
    };
    let mut deposit_amount:u128=0;
    if deposit_info!=vec![]{
        for mut current_deposit_coin in deposit_info.clone(){
                for  new_deposit_coin in info.funds.clone(){
                    if new_deposit_coin.denom==current_deposit_coin.denom{
                        current_deposit_coin.amount=current_deposit_coin.amount+new_deposit_coin.amount;
                        deposit_amount=new_deposit_coin.amount.u128();
                        }
                    }
                }
    }
    else 
    {   let mut is_correct_fund=false;
        for deposit_iter in info.funds.clone(){
            if prop.token_denom==deposit_iter.denom
            {
                deposit_amount=deposit_iter.amount.u128();
                is_correct_fund=true;
            }
        }
        if is_correct_fund{
            deposit_info=info.funds;
        }
        else {
            return Err(ContractError::IncorrectDenomDeposit {})
        }
    }

    prop.current_deposit=prop.current_deposit+deposit_amount;

    if Uint128::from(prop.current_deposit)>Uint128::from(prop.min_deposit)
    {
        prop.status=Status::Open
    }


    VOTERDEPOSIT.save(deps.storage, (proposal_id, &info.sender), &deposit_info)?;
    PROPOSALS.save(deps.storage, proposal_id, &prop)?;

    PROPOSALS.save(deps.storage, proposal_id, &prop)?;
    
    Ok(Response::new()
        
        .add_attribute("action", "refund")
        .add_attribute("sender", info.sender)
        .add_attribute("proposal_id", proposal_id.to_string()))
}
pub fn execute_refund(
    deps: DepsMut<ComdexQuery>,
    env: Env,
    info: MessageInfo,
    proposal_id: u64,
) -> Result<Response<ComdexMessages>, ContractError> {

    let   prop = PROPOSALS.load(deps.storage, proposal_id)?;
    let status = prop.current_status(&env.block);
    if [ Status::Pending,Status::Rejected,Status::Open]
        .iter()
        .any(|x| *x == status)
    {
        return Err(ContractError::NonPassedProposalRefund {});
    }
    
    if !prop.expires.is_expired(&env.block) {
        return Err(ContractError::NotExpired {});
    
    }

    let deposit_info =  VOTERDEPOSIT.may_load(deps.storage, (proposal_id, &info.sender))?;
    
    if let None=deposit_info
    {
        return Err(ContractError::NoDeposit {});
    }

    //// need to update current_deposit////////
    
    VOTERDEPOSIT.remove(deps.storage, (proposal_id, &info.sender));

    PROPOSALS.save(deps.storage, proposal_id, &prop)?;
    
    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount:deposit_info.unwrap()
        })
        .add_attribute("action", "refund")
        .add_attribute("sender", info.sender)
        .add_attribute("proposal_id", proposal_id.to_string()))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ComdexQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {

        QueryMsg::Threshold {proposal_id} => to_binary(&query_threshold(deps,proposal_id)?),
        QueryMsg::Proposal { proposal_id } => to_binary(&query_proposal_detailed(deps, env, proposal_id)?),
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
        QueryMsg::ListAppProposal { app_id } => to_binary(&get_proposals_by_app(deps, env,app_id)?),
        QueryMsg::AppAllUpData { app_id } => to_binary(&get_all_up_info_by_app(deps,env, app_id)?),

        
    }
}




fn query_threshold(deps: Deps<ComdexQuery>,proposal_id:u64) -> StdResult<ThresholdResponse> {
    let cfg = CONFIG.load(deps.storage)?;
    let prop = PROPOSALS.load(deps.storage, proposal_id)?;

    Ok(cfg.threshold.to_response(prop.total_weight))
}

fn query_proposal_detailed(deps: Deps<ComdexQuery>, env: Env, id: u64) -> StdResult<ProposalResponseTotal> {
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
        votes:prop.votes,
        duration:prop.duration,
        start_height:prop.start_height,
        threshold:prop.threshold,
        proposer:prop.proposer,
        token_denom:prop.token_denom,
        total_weight:prop.total_weight,
        current_deposit:prop.current_deposit,
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

    Ok(ProposalListResponse{ proposals })
}

fn get_proposals_by_app(deps: Deps<ComdexQuery>,env:Env,app_id : u64) -> StdResult<Vec<ProposalResponseTotal>> {
    let info= match PROPOSALSBYAPP.may_load(deps.storage,app_id )?{ 
            Some(record) => record,
            None => vec![]
    };

    let mut all_proposals=vec![];

    for i in info
    {
        let proposal=query_proposal_detailed(deps,env.clone(),i)?;
        all_proposals.push(proposal);
    }


    Ok(all_proposals)
}




fn get_all_up_info_by_app(deps: Deps<ComdexQuery>,env:Env,app_id : u64) -> StdResult<AppGovConfig> {
    let info= match PROPOSALSBYAPP.may_load(deps.storage,app_id )?{ 
        Some(record) => record,
        None => vec![]
};
    let mut participation_info= APPGOVCONFIG.may_load(deps.storage,app_id )?.unwrap();
    let mut total_votes_weight:u128=0;
    for i in info
    {
        let proposal=query_proposal_detailed(deps,env.clone(),i)?;
         total_votes_weight=total_votes_weight+proposal.votes.total();
    }
    participation_info.active_participation_supply=total_votes_weight;


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

