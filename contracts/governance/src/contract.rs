use std::cmp::Ordering;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, BlockInfo, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdResult,BankMsg,Coin,QueryRequest, Uint128,
};
use crate::coin_helpers::assert_sent_sufficient_coin;
use comdex_bindings::ComdexMessages;
use comdex_bindings::{ComdexQuery,StateResponse,MessageValidateResponse};
use cw2::set_contract_version;
use cw3::{
    ProposalListResponse, ProposalResponse, Status, Vote, VoteInfo, VoteListResponse, VoteResponse,
    };
use cw_storage_plus::Bound;
use cw_utils::{Expiration, ThresholdResponse,Duration,Threshold};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{next_id, Ballot, Config, Proposal, Votes, BALLOTS, CONFIG, PROPOSALS,PROPOSALSBYAPP};
use crate::validation::{whitelistassetlocker,query_owner_token_at_height,query_app_exists,get_token_supply,query_get_asset_data,
    whitelistassetlockerrewards,whitelistappidvaultinterest,validate_threshold,addextendedpairvault,collectorlookuptable};
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
        ExecuteMsg::Test { msg } => execute_execute1(deps, env, info, msg),

    }
}

pub fn execute_execute1(
    _deps: DepsMut<ComdexQuery>,
    _env: Env,
    info: MessageInfo,
    msg:ComdexMessages
) -> Result<Response<ComdexMessages>, ContractError> {
    // anyone can trigger this if the vote passed

    

    // dispatch all proposed messages
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "execute")
        .add_attribute("sender", info.sender)
        )
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

    assert_sent_sufficient_coin(&info.funds,  Some(min_deposit))?;

    //check if gov denom exists in user deposit

    let mut is_correct_denom=false;
    for user_deposit in &info.funds{
        if user_deposit.denom.eq(&gov_token_denom){
            is_correct_denom=true;
        }
    }

    if !is_correct_denom{
        return Err(ContractError::DenomNotFound {})
    }

    for msg in msgs.clone()
    {
        match msg{
            ComdexMessages::MsgWhiteListAssetLocker{app_mapping_id,asset_id}=>whitelistassetlocker(deps.as_ref(),app_mapping_id,asset_id,app_id)?,
            ComdexMessages::MsgWhitelistAppIdLockerRewards{app_mapping_id,asset_id}=>whitelistassetlockerrewards(deps.as_ref(),app_mapping_id,asset_id,app_id)?,
            ComdexMessages::MsgWhitelistAppIdVaultInterest{app_mapping_id}=>whitelistappidvaultinterest(deps.as_ref(),app_mapping_id,app_id)?,
            ComdexMessages::MsgAddExtendedPairsVault{app_mapping_id,pair_id,liquidation_ratio:_,
            stability_fee,closing_fee,liquidation_penalty:_,draw_down_fee,
            is_vault_active:_,debt_ceiling,debt_floor,is_psm_pair:_,
            min_cr:_,pair_name,asset_out_oracle_price:_,assset_out_price:_}=>addextendedpairvault(deps.as_ref(),app_mapping_id,
            pair_id,
            stability_fee,
            closing_fee,
            draw_down_fee,
            debt_ceiling,
            debt_floor,
            pair_name,app_id)?,
            ComdexMessages::MsgSetCollectorLookupTable{app_mapping_id ,
                collector_asset_id ,
                secondary_asset_id ,
                surplus_threshold:_ ,
                debt_threshold:_,
                locker_saving_rate:_,
                lot_size:_ ,
                bid_factor:_} =>collectorlookuptable(deps.as_ref(),app_mapping_id,collector_asset_id,secondary_asset_id,app_id)?,
                

        }
    }
    // create a proposal
    let mut prop = Proposal {
        title,
        description,
        start_height: env.block.height,
        expires,
        msgs,
        status: Status::Open,
        votes: Votes::yes(voting_power.amount.u128()) ,
        threshold: cfg.threshold,
        total_weight: Uint128::from(total_weight).u128(),
        deposit:info.funds,
        proposer :info.sender.to_string(),
        token_denom : gov_token_denom,
        deposit_refunded:false

    };

    

    
    prop.update_status(&env.block);
    
    //get latest proposal id counter
    let id = next_id(deps.storage)?;
    PROPOSALS.save(deps.storage, id, &prop)?;

    // add the first yes vote from voter
    let ballot = Ballot {
        weight: voting_power.amount.u128(),
        vote: Vote::Yes,
    };
    BALLOTS.save(deps.storage, (id, &info.sender), &ballot)?;
    let  propbyapp = match PROPOSALSBYAPP.may_load(deps.storage, app_id)?
    {Some(data)=>Some(data),
        None=>Some(vec![])};

    let mut app_proposals=propbyapp.unwrap();
    app_proposals.push(id);
    PROPOSALSBYAPP.save(deps.storage, app_id, &app_proposals)?;
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
    if prop.status != Status::Open {
        return Err(ContractError::NotOpen {});
    }
    if prop.expires.is_expired(&env.block) {
        return Err(ContractError::Expired {});
    }

    //Get Proposal Start Height
    let proposal_height=prop.start_height;
    let cfg = CONFIG.load(deps.storage)?;
    let token_denom=&prop.token_denom;
    //Get Voter power at proposal height 
    let voting_power=query_owner_token_at_height(deps.as_ref(),info.sender.to_string(),token_denom.to_string(),proposal_height.to_string(),cfg.target)?;


    // cast vote if no vote previously cast
    BALLOTS.update(deps.storage, (proposal_id, &info.sender), |bal| match bal {
        Some(_) => Err(ContractError::AlreadyVoted {}),
        None => Ok(Ballot {
            weight: voting_power.amount.u128(),
            vote,
        }),
    })?;

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
    // anyone can trigger this if the vote passed

    let mut prop = PROPOSALS.load(deps.storage, proposal_id)?;
    // we allow execution even after the proposal "expiration" as long as all vote come in before
    // that point. If it was approved on time, it can be executed any time.
    if prop.status != Status::Passed {
        return Err(ContractError::WrongExecuteStatus {});
    }

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

pub fn execute_refund(
    deps: DepsMut<ComdexQuery>,
    env: Env,
    info: MessageInfo,
    proposal_id: u64,
) -> Result<Response<ComdexMessages>, ContractError> {

    let mut  prop = PROPOSALS.load(deps.storage, proposal_id)?;
    if ![Status::Executed, Status::Rejected, Status::Passed]
        .iter()
        .any(|x| *x != prop.status)
    {
        return Err(ContractError::WrongRefundStatus {});
    }
    if !prop.expires.is_expired(&env.block) {
        return Err(ContractError::NotExpired {});
    }
    if info.sender!=prop.proposer{
        return Err(ContractError::Unauthorized {});
    }

    if prop.deposit_refunded{
        return Err(ContractError::NotExpired {})
    }


    prop.deposit_refunded=true;
    PROPOSALS.save(deps.storage, proposal_id, &prop)?;
    

    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: prop.proposer.clone().into(),
            amount:prop.deposit
        })
        .add_attribute("action", "refund")
        .add_attribute("sender", info.sender)
        .add_attribute("proposal_id", proposal_id.to_string()))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ComdexQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Test {query} => to_binary(&get_test(deps,query)?),

        QueryMsg::Threshold {proposal_id} => to_binary(&get_token_supply1(deps)?),
        QueryMsg::Proposal { proposal_id } => to_binary(&query_proposal(deps, env, proposal_id)?),
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
        QueryMsg::ListAppProposal { proposal_id } => to_binary(&get_proposals_by_app(deps, proposal_id)?),
        
    }
}
fn get_test(deps: Deps<ComdexQuery>,query:ComdexQuery) -> StdResult<MessageValidateResponse> {
    let voting_power=deps
    .querier
    .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;
    
    Ok(voting_power)
}

 fn get_token_supply1(deps: Deps<ComdexQuery>) -> StdResult<StateResponse> {
    let voting_power=deps
    .querier
    .query::<StateResponse>(&QueryRequest::Custom(
        ComdexQuery::State {address: "comdex1chgn3mkwe646jcd9lrupwtf5f6p6930lzmsa7w".to_string(), denom: "ucmdx".to_string(),height:"3000".to_string(),target:"0.0.0.0:9090".to_string()}
    ))?;
    
    Ok(voting_power)
}

fn query_threshold(deps: Deps<ComdexQuery>,proposal_id:u64) -> StdResult<ThresholdResponse> {
    let cfg = CONFIG.load(deps.storage)?;
    let prop = PROPOSALS.load(deps.storage, proposal_id)?;

    Ok(cfg.threshold.to_response(prop.total_weight))
}


fn query_proposal(deps: Deps<ComdexQuery>, env: Env, id: u64) -> StdResult<ProposalResponse> {
    let prop = PROPOSALS.load(deps.storage, id)?;
    let status = prop.current_status(&env.block);
    let threshold = prop.threshold.to_response(prop.total_weight);
    Ok(ProposalResponse {
        id,
        title: prop.title,
        description: prop.description,
        msgs: prop.msgs,
        status,
        expires: prop.expires,
        threshold,
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

fn get_proposals_by_app(deps: Deps<ComdexQuery>,proposal_id: u64) -> StdResult<Vec<u64>> {
    let info= match PROPOSALSBYAPP.may_load(deps.storage,proposal_id )?{ 
            Some(record) => Some(record),
            None => Some(vec![]) 
    };

    Ok(info.unwrap())
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



