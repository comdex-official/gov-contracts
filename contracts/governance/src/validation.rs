
#[cfg(not(feature = "library"))]
use cosmwasm_std::{ Deps, StdResult,Coin,QueryRequest,Decimal 
};
use comdex_bindings::{ComdexQuery,StateResponse,GetAppResponse,GetAssetDataResponse,TotalSupplyResponse,MessageValidateResponse};
use crate::error::ContractError;

pub fn validate_threshold(threshold:&Decimal,quorum:&Decimal)
-> Result< (), ContractError>
{
    if *threshold > Decimal::percent(100) || *threshold < Decimal::percent(50) {
        Err(ContractError::InvalidThreshold {})
    } 
    else if quorum.is_zero() {
        Err(ContractError::ZeroQuorumThreshold {})
    } else if *quorum > Decimal::one() {
        Err(ContractError::UnreachableQuorumThreshold {})
    }
    else {
        Ok(())
    }
    
}

//check if whitelist asset locker proposal pass the eligibility
pub fn whitelistassetlocker(deps:Deps<ComdexQuery>,app_mapping_id:u64,asset_id:u64,app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::WhitelistAppIdLockerRewards{app_id :app_mapping_id, asset_id:vec![asset_id]};
    let query_result= deps
    .querier
    .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found{
        Ok(())

    }
    else {
        let err=query_result.err;
        return Err(ContractError::ProposalError {err})
    }
    
}

pub fn whitelistassetlockerrewards(deps:Deps<ComdexQuery>,app_mapping_id:u64,asset_id:Vec<u64>,app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::WhitelistAppIdLockerRewards{app_id :app_mapping_id, asset_id:asset_id};
    let query_result= deps
    .querier
    .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found{
        Ok(())
    }
    else {
        let err=query_result.err;
        return Err(ContractError::ProposalError {err})
    }
    
}

pub fn whitelistappidvaultinterest(deps:Deps<ComdexQuery>,app_mapping_id:u64,app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::WhitelistAppIdVaultInterest{app_id :app_mapping_id};
    let query_result= deps
    .querier
    .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found{
        Ok(())

    }
    else {
        let err=query_result.err;
        return Err(ContractError::ProposalError {err})
    }
    
}




pub fn query_owner_token_at_height(deps: Deps<ComdexQuery>,address:String,denom:String,height:String,target:String) -> StdResult<Coin> {
    let voting_power=deps
    .querier
    .query::<StateResponse>(&QueryRequest::Custom(
        ComdexQuery::State {address: address, denom: denom,height:height,target:target}
    ))?.amount;
    
    Ok(voting_power)
}

pub fn query_app_exists(deps: Deps<ComdexQuery>,app_mapping_id_1:u64) -> StdResult<GetAppResponse> {
    let app_info=deps
    .querier
    .query::<GetAppResponse>(&QueryRequest::Custom(
        ComdexQuery::GetApp {app_mapping_id: app_mapping_id_1}
    ))?;
    
    Ok(app_info)
}

pub fn query_get_asset_data(deps: Deps<ComdexQuery>,asset_id:u64) -> StdResult<String> {
    let asset_denom=deps
    .querier
    .query::<GetAssetDataResponse>(&QueryRequest::Custom(
        ComdexQuery::GetAssetData {asset_id: asset_id}
    ))?;
    
    Ok(asset_denom.denom)
}



pub fn get_token_supply(deps: Deps<ComdexQuery>,app_id:u64,asset_id:u64) -> StdResult<u64> {
    let total_token_supply=deps
    .querier
    .query::<TotalSupplyResponse>(&QueryRequest::Custom(
        ComdexQuery::TotalSupply {app_id:app_id,asset_id: asset_id}
    ))?;
    
    Ok(total_token_supply.current_supply)
}



