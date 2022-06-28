use crate::error::ContractError;
use comdex_bindings::{
    ComdexQuery, GetAppResponse, GetAssetDataResponse, MessageValidateResponse, StateResponse,
    TotalSupplyResponse,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{Coin, Decimal, Deps, QueryRequest, StdResult};

pub fn validate_threshold(threshold: &Decimal, quorum: &Decimal) -> Result<(), ContractError> {
    if *threshold > Decimal::percent(100) || *threshold < Decimal::percent(50) {
        Err(ContractError::InvalidThreshold {})
    } else if quorum.is_zero() {
        Err(ContractError::ZeroQuorumThreshold {})
    } else if *quorum > Decimal::one() {
        Err(ContractError::UnreachableQuorumThreshold {})
    } else {
        Ok(())
    }
}

/// validate checks to update vault stability fee
pub fn update_pairvault_stability(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
    ext_pair_id_param: u64,
    app_id: u64,
) -> Result<(), ContractError> {
    if app_mapping_id_param != app_id {
        return Err(ContractError::DifferentAppID {});
    }
    let query = ComdexQuery::UpdatePairsVaultQuery {
        app_mapping_id: app_mapping_id_param,
        ext_pair_id: ext_pair_id_param,
    };
    let query_result = deps
        .querier
        .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found {
        Ok(())
    } else {
        let err = query_result.err;
        Err(ContractError::ProposalError { err })
    }
}

/// validate checks to update locker saving rate
pub fn update_locker_lsr(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
    asset_id_param: u64,
    app_id: u64,
) -> Result<(), ContractError> {
    if app_mapping_id_param != app_id {
        return Err(ContractError::DifferentAppID {});
    }
    let query = ComdexQuery::UpdateCollectorLookupTableQuery {
        app_mapping_id: app_mapping_id_param,
        asset_id: asset_id_param,
    };
    let query_result = deps
        .querier
        .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found {
        Ok(())
    } else {
        let err = query_result.err;
        Err(ContractError::ProposalError { err })
    }
}

pub fn remove_whitelist_asset_locker(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
    asset_id_param: u64,
    app_id: u64,
) -> Result<(), ContractError> {
    if app_mapping_id_param != app_id {
        return Err(ContractError::DifferentAppID {});
    }
    let query = ComdexQuery::RemoveWhitelistAssetLockerQuery {
        app_mapping_id: app_mapping_id_param,
        asset_id: asset_id_param,
    };
    let query_result = deps
        .querier
        .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found {
        Ok(())
    } else {
        let err = query_result.err;
        Err(ContractError::ProposalError { err })
    }
}

pub fn remove_whitelist_app_id_vault_interest(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
    app_id: u64,
) -> Result<(), ContractError> {
    if app_mapping_id_param != app_id {
        return Err(ContractError::DifferentAppID {});
    }
    let query = ComdexQuery::RemoveWhitelistAppIdVaultInterestQuery {
        app_mapping_id: app_mapping_id_param,
    };
    let query_result = deps
        .querier
        .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found {
        Ok(())
    } else {
        let err = query_result.err;
        Err(ContractError::ProposalError { err })
    }
}

// Validation check to whitelist an app for liquidation
pub fn whitelist_app_id_liquidation(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
    app_id: u64,
) -> Result<(), ContractError> {
    if app_mapping_id_param != app_id {
        return Err(ContractError::DifferentAppID {});
    }
    let query = ComdexQuery::WhitelistAppIdLiquidationQuery {
        app_mapping_id: app_mapping_id_param,
    };
    let query_result = deps
        .querier
        .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found {
        Ok(())
    } else {
        let err = query_result.err;
        Err(ContractError::ProposalError { err })
    }
}

// Validation check to remove whitelisted  app for liquidation

pub fn remove_whitelist_app_id_liquidation(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
    app_id: u64,
) -> Result<(), ContractError> {
    if app_mapping_id_param != app_id {
        return Err(ContractError::DifferentAppID {});
    }
    let query = ComdexQuery::RemoveWhitelistAppIdLiquidationQuery {
        app_mapping_id: app_mapping_id_param,
    };
    let query_result = deps
        .querier
        .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found {
        Ok(())
    } else {
        let err = query_result.err;
        Err(ContractError::ProposalError { err })
    }
}

//check asset is available for rewards in locker
pub fn whitelist_asset_locker_rewards(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
    asset_id_param: Vec<u64>,
    app_id: u64,
) -> Result<(), ContractError> {
    if app_mapping_id_param != app_id {
        return Err(ContractError::DifferentAppID {});
    }
    let query = ComdexQuery::WhitelistAppIdLockerRewards {
        app_mapping_id: app_mapping_id_param,
        asset_id: asset_id_param,
    };
    let query_result = deps
        .querier
        .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found {
        Ok(())
    } else {
        let err = query_result.err;
        Err(ContractError::ProposalError { err })
    }
}

/// check if asset and be whitelisted in locker
pub fn whitelist_asset_locker_eligible(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
    asset_id_param: u64,
    app_id: u64,
) -> Result<(), ContractError> {
    if app_mapping_id_param != app_id {
        return Err(ContractError::DifferentAppID {});
    }
    let query = ComdexQuery::WhiteListedAssetQuery {
        app_mapping_id: app_mapping_id_param,
        asset_id: asset_id_param,
    };
    let query_result = deps
        .querier
        .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found {
        Ok(())
    } else {
        let err = query_result.err;
        Err(ContractError::ProposalError { err })
    }
}

/// check if mapping is there in collector lookup for thr app and asset
pub fn collector_lookup_table(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
    collector_asset_id_param: u64,
    secondary_asset_id_param: u64,
    app_id: u64,
) -> Result<(), ContractError> {
    if app_mapping_id_param != app_id {
        return Err(ContractError::DifferentAppID {});
    }
    let query = ComdexQuery::CollectorLookupTableQuery {
        app_mapping_id: app_mapping_id_param,
        collector_asset_id: collector_asset_id_param,
        secondary_asset_id: secondary_asset_id_param,
    };
    let query_result = deps
        .querier
        .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found {
        Ok(())
    } else {
        let err = query_result.err;
        Err(ContractError::ProposalError { err })
    }
}

//// check mapping for auction for an app
pub fn auction_mapping_for_app(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
    app_id: u64,
) -> Result<(), ContractError> {
    if app_mapping_id_param != app_id {
        return Err(ContractError::DifferentAppID {});
    }
    let query = ComdexQuery::AuctionMappingForAppQuery {
        app_mapping_id: app_mapping_id_param,
    };
    let query_result = deps
        .querier
        .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found {
        Ok(())
    } else {
        let err = query_result.err;
        Err(ContractError::ProposalError { err })
    }
}
//// eligibility checks to add and extended pair  vaults
pub fn add_extended_pair_vault(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
    pair_id_param: u64,
    stability_fee_param: Decimal,
    closing_fee_param: Decimal,
    draw_down_fee_param: Decimal,
    debt_ceiling_param: u64,
    debt_floor_param: u64,
    pair_name_param: String,
    app_id: u64,
) -> Result<(), ContractError> {
    if app_mapping_id_param != app_id {
        return Err(ContractError::DifferentAppID {});
    }
    let query = ComdexQuery::ExtendedPairsVaultRecordsQuery {
        app_mapping_id: app_mapping_id_param,
        pair_id: pair_id_param,
        stability_fee: stability_fee_param,
        closing_fee: closing_fee_param,
        draw_down_fee: draw_down_fee_param,
        debt_ceiling: debt_ceiling_param,
        debt_floor: debt_floor_param,
        pair_name: pair_name_param,
    };
    let query_result = deps
        .querier
        .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found {
        Ok(())
    } else {
        let err = query_result.err;
        Err(ContractError::ProposalError { err })
    }
}

/// checks for activating vault interest for an app
pub fn whitelist_app_id_vault_interest(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
    app_id: u64,
) -> Result<(), ContractError> {
    if app_mapping_id_param != app_id {
        return Err(ContractError::DifferentAppID {});
    }
    let query = ComdexQuery::WhitelistAppIdVaultInterest {
        app_mapping_id: app_mapping_id_param,
    };
    let query_result = deps
        .querier
        .query::<MessageValidateResponse>(&QueryRequest::Custom(query))?;

    if query_result.found {
        Ok(())
    } else {
        let err = query_result.err;
        Err(ContractError::ProposalError { err })
    }
}

/// query token balance of a user for a denom at a specific height
pub fn query_owner_token_at_height(
    deps: Deps<ComdexQuery>,
    address_param: String,
    denom_param: String,
    height_param: String,
    target_param: String,
) -> StdResult<Coin> {
    let voting_power = deps
        .querier
        .query::<StateResponse>(&QueryRequest::Custom(ComdexQuery::State {
            address: address_param,
            denom: denom_param,
            height: height_param,
            target: target_param,
        }))?
        .amount;

    Ok(voting_power)
}

//// check get app date
pub fn query_app_exists(
    deps: Deps<ComdexQuery>,
    app_mapping_id_param: u64,
) -> StdResult<GetAppResponse> {
    let app_info =
        deps.querier
            .query::<GetAppResponse>(&QueryRequest::Custom(ComdexQuery::GetApp {
                app_mapping_id: app_mapping_id_param,
            }))?;

    Ok(app_info)
}

/// get asset data for an asset_id
pub fn query_get_asset_data(deps: Deps<ComdexQuery>, asset_id_param: u64) -> StdResult<String> {
    let asset_denom = deps
        .querier
        .query::<GetAssetDataResponse>(&QueryRequest::Custom(ComdexQuery::GetAssetData {
            asset_id: asset_id_param,
        }))?;

    Ok(asset_denom.denom)
}

/// get token_supply of an asset at current height
pub fn get_token_supply(
    deps: Deps<ComdexQuery>,
    app_id_param: u64,
    asset_id_param: u64,
) -> StdResult<u64> {
    let total_token_supply = deps
        .querier
        .query::<TotalSupplyResponse>(&QueryRequest::Custom(ComdexQuery::TotalSupply {
            app_mapping_id: app_id_param,
            asset_id: asset_id_param,
        }))?;

    Ok(total_token_supply.current_supply)
}
