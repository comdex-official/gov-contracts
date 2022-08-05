use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Coin, Deps, Env, MessageInfo, StdError, StdResult, DepsMut,
};

use crate::msg::{
    IssuedNftResponse, IssuedVtokensResponse, LockedTokensResponse, QueryMsg,
    UnlockedTokensResponse, UnlockingTokensResponse,
};
use crate::state::{Status, LOCKED, TOKENS, UNLOCKED, UNLOCKING, VTOKENS};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, info: MessageInfo, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IssuedNft { address } => to_binary(&query_issued_nft(deps, env, info, address)?),

        QueryMsg::UnlockedTokens { address, denom } => {
            to_binary(&query_unlocked_tokens(deps, env, info, address, denom)?)
        }

        QueryMsg::UnlockingTokens { address, denom } => {
            to_binary(&query_unlocking_tokens(deps, env, info, address, denom)?)
        }

        QueryMsg::LockedTokens { address, denom } => {
            to_binary(&query_locked_tokens(deps, env, info, address, denom)?)
        }

        QueryMsg::IssuedVtokens { address } => {
            to_binary(&query_issued_vtokens(deps, env, info, address)?)
        }
    }
}

pub fn query_issued_nft(
    deps: Deps,
    _env: Env,
    _info: MessageInfo,
    address: String,
) -> StdResult<IssuedNftResponse> {
    let owner = deps.api.addr_validate(&address)?;
    let nft = TOKENS.may_load(deps.storage, owner)?;

    match nft {
        Some(val) => Ok(IssuedNftResponse { nft: val }),
        None => Err(StdError::NotFound {
            kind: String::from("NFT does not exist for the given address"),
        }),
    }
}

pub fn query_unlocked_tokens(
    deps: Deps,
    _env: Env,
    info: MessageInfo,
    address: Option<String>,
    denom: Option<String>,
) -> StdResult<UnlockedTokensResponse> {
    // set `owner` for querying tokens
    let owner: Addr;
    if let Some(val) = address {
        owner = deps.api.addr_validate(&val)?;
    } else {
        owner = info.sender;
    };

    // result contains either a single token for the given denom or all
    // unlocked tokens for the given owner
    let vtokens: Vec<Coin>;
    if let Some(denom) = denom {
        let res = VTOKENS.load(deps.storage, (owner, &denom))?;
        if res.status != Status::Unlocked {
            return Err(StdError::GenericErr {
                msg: String::from("No unlocked tokens for given denom"),
            });
        }
        vtokens = vec![res.vtoken];
    } else {
        vtokens = UNLOCKED.load(deps.storage, owner)?;
    }

    Ok(UnlockedTokensResponse { tokens: vtokens })
}

pub fn query_locked_tokens(
    deps: Deps,
    _env: Env,
    info: MessageInfo,
    address: Option<String>,
    denom: Option<String>,
) -> StdResult<LockedTokensResponse> {
    // set `owner` for querying tokens
    let owner: Addr;
    if let Some(val) = address {
        owner = deps.api.addr_validate(&val)?;
    } else {
        owner = info.sender;
    };

    // result contains either a single token for the given denom or all
    // locked tokens for the given owner
    let vtokens: Vec<Coin>;
    if let Some(denom) = denom {
        let res = VTOKENS.load(deps.storage, (owner, &denom))?;
        if res.status != Status::Locked {
            return Err(StdError::GenericErr {
                msg: String::from("No locked tokens for given denom"),
            });
        }
        vtokens = vec![res.vtoken];
    } else {
        vtokens = LOCKED.load(deps.storage, owner)?;
    }

    Ok(LockedTokensResponse { tokens: vtokens })
}

pub fn query_unlocking_tokens(
    deps: Deps,
    _env: Env,
    info: MessageInfo,
    address: Option<String>,
    denom: Option<String>,
) -> StdResult<UnlockingTokensResponse> {
    // set `owner` for querying tokens
    let owner: Addr;
    if let Some(val) = address {
        owner = deps.api.addr_validate(&val)?;
    } else {
        owner = info.sender;
    };

    // result contains either a single token for the given denom or all
    // locked tokens for the given owner
    let vtokens: Vec<Coin>;
    if let Some(denom) = denom {
        let res = VTOKENS.load(deps.storage, (owner, &denom))?;
        if res.status != Status::Unlocking {
            return Err(StdError::GenericErr {
                msg: String::from("No unlocking tokens for given denom"),
            });
        }
        vtokens = vec![res.vtoken];
    } else {
        vtokens = UNLOCKING.load(deps.storage, owner)?;
    }

    Ok(UnlockingTokensResponse { tokens: vtokens })
}

#[allow(unused_variables)]
pub fn query_issued_vtokens(
    deps: Deps,
    env: Env,
    info: MessageInfo,
    address: Option<String>,
) -> StdResult<IssuedVtokensResponse> {
    Err(StdError::GenericErr {
        msg: "Not implemented".into(),
    })
}
