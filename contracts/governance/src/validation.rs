
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

/// validate checks to update vault stability fee
pub fn updatepairvaultstability(deps:Deps<ComdexQuery>,app_mapping_id:u64,ext_pair_id:u64,app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::UpdatePairsVaultQuery{app_mapping_id :app_mapping_id,ext_pair_id:ext_pair_id};
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

/// validate checks to update locker saving rate
pub fn updatelockerlsr(deps:Deps<ComdexQuery>,app_mapping_id:u64,asset_id:u64,app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::UpdateCollectorLookupTableQuery{app_mapping_id :app_mapping_id,asset_id:asset_id};
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

pub fn removewhitelistassetlocker(deps:Deps<ComdexQuery>,app_mapping_id:u64,asset_id:u64,app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::RemoveWhitelistAssetLockerQuery{app_mapping_id :app_mapping_id,asset_id:asset_id};
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

pub fn removewhitelistappidvaultinterest(deps:Deps<ComdexQuery>,app_mapping_id:u64,
    app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::RemoveWhitelistAppIdVaultInterestQuery{app_mapping_id :app_mapping_id};
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

// Validation check to whitelist an app for liquidation
pub fn whitelistappidliquidation(deps:Deps<ComdexQuery>,app_mapping_id:u64,
    app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::WhitelistAppIdLiquidationQuery{app_mapping_id :app_mapping_id};
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

// Validation check to remove whitelisted  app for liquidation

pub fn removewhitelistappidliquidation(deps:Deps<ComdexQuery>,app_mapping_id:u64,
    app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::RemoveWhitelistAppIdLiquidationQuery{app_mapping_id :app_mapping_id};
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



//check asset is available for rewards in locker
pub fn whitelistassetlockerrewards(deps:Deps<ComdexQuery>,app_mapping_id:u64,asset_id:Vec<u64>,app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::WhitelistAppIdLockerRewards{app_mapping_id :app_mapping_id, asset_id:asset_id};
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

/// check if asset and be whitelisted in locker
pub fn whitelistassetlockereligible(deps:Deps<ComdexQuery>,app_mapping_id:u64,asset_id:u64,app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::WhiteListedAssetQuery{app_mapping_id :app_mapping_id, asset_id:asset_id};
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

/// check if mapping is there in collector lookup for thr app and asset
pub fn collectorlookuptable(deps:Deps<ComdexQuery>,app_mapping_id:u64,
    collector_asset_id:u64,
    secondary_asset_id:u64,
    app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::CollectorLookupTableQuery{app_mapping_id :app_mapping_id, collector_asset_id:collector_asset_id,secondary_asset_id:secondary_asset_id};
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

//// check mapping for auction for an app
pub fn auctionmappingforapp(deps:Deps<ComdexQuery>,app_mapping_id:u64,
    app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::AuctionMappingForAppQuery{app_mapping_id :app_mapping_id};
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
//// eligibility checks to add and extended pair  vaults
pub fn addextendedpairvault(deps:Deps<ComdexQuery>,app_mapping_id :u64,
    pair_id:u64,
    stability_fee:Decimal,
    closing_fee:Decimal,
    draw_down_fee:Decimal,
    debt_ceiling:u64,
    debt_floor:u64,
    pair_name:String,app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::ExtendedPairsVaultRecordsQuery{app_mapping_id :app_mapping_id, pair_id:pair_id,stability_fee:stability_fee,
    closing_fee:closing_fee,draw_down_fee:draw_down_fee,debt_ceiling:debt_ceiling,debt_floor:debt_floor,pair_name};
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


/// checks for activating vault interest for an app
pub fn whitelistappidvaultinterest(deps:Deps<ComdexQuery>,app_mapping_id:u64,app_id:u64)
-> Result< (), ContractError>
{
    if app_mapping_id!=app_id
    {
        return Err(ContractError::DifferentAppID {})
    }
    let query= ComdexQuery::WhitelistAppIdVaultInterest{app_mapping_id :app_mapping_id};
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



/// query token balance of a user for a denom at a specific height
pub fn query_owner_token_at_height(deps: Deps<ComdexQuery>,address:String,denom:String,height:String,target:String) -> StdResult<Coin> {
    let voting_power=deps
    .querier
    .query::<StateResponse>(&QueryRequest::Custom(
        ComdexQuery::State {address: address, denom: denom,height:height,target:target}
    ))?.amount;
    
    Ok(voting_power)
}

//// check get app date 
pub fn query_app_exists(deps: Deps<ComdexQuery>,app_mapping_id_1:u64) -> StdResult<GetAppResponse> {
    let app_info=deps
    .querier
    .query::<GetAppResponse>(&QueryRequest::Custom(
        ComdexQuery::GetApp {app_mapping_id: app_mapping_id_1}
    ))?;
    
    Ok(app_info)
}

/// get asset data for an asset_id
pub fn query_get_asset_data(deps: Deps<ComdexQuery>,asset_id:u64) -> StdResult<String> {
    let asset_denom=deps
    .querier
    .query::<GetAssetDataResponse>(&QueryRequest::Custom(
        ComdexQuery::GetAssetData {asset_id: asset_id}
    ))?;
    
    Ok(asset_denom.denom)
}


/// get token_supply of an asset at current height
pub fn get_token_supply(deps: Deps<ComdexQuery>,app_id:u64,asset_id:u64) -> StdResult<u64> {
    let total_token_supply=deps
    .querier
    .query::<TotalSupplyResponse>(&QueryRequest::Custom(
        ComdexQuery::TotalSupply {app_mapping_id:app_id,asset_id: asset_id}
    ))?;
    
    Ok(total_token_supply.current_supply)
}



