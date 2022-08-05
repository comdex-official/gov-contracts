use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, Env, MessageInfo, StdError, StdResult, DepsMut,
};

use crate::ContractError;
// use crate::error::ContractError;
use crate::msg::{GetUnlockedTokenRespose, IssuedTokensResponse, QueryMsg};
use crate::state::{Status, TOKENS, VTOKENS};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: DepsMut, env: Env, info: MessageInfo, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IssuedTokens { address, token_id } => {
            to_binary(&query_issued_tokens(deps, env, address, token_id)?)
        }
        QueryMsg::GetUnlockedTokens { denom } => {
            to_binary(&get_unlocked_tokens(deps, info, denom)?)
        }
    }
}

fn query_issued_tokens(
    deps: DepsMut,
    _env: Env,
    address: String,
    token_id: u64,
) -> StdResult<IssuedTokensResponse> {
    let owner = deps.api.addr_validate(&address[..])?;
    let token = TOKENS.may_load(deps.storage, owner)?;

    match token {
        Some(val) => Ok(IssuedTokensResponse {
            vtokens: val.vtokens,
        }),
        None => {
            let err = format!("NFT not found for owner: {address} and token ID: {token_id}");
            Err(StdError::NotFound { kind: err })
        }
    }
}

pub fn get_unlocked_tokens(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
) -> StdResult<GetUnlockedTokenRespose> {
    let Vtoken = VTOKENS.load(deps.storage, (info.sender, &denom)).unwrap();

    if Vtoken.status != Status::Unlocked {
        ContractError::NotUnlocked {};
    }

    let res = GetUnlockedTokenRespose {
        tokens: Vtoken.token.amount,
    };
    Ok(res)
}
