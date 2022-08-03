use cosmwasm_std::{entry_point, to_binary, Binary, Deps, Env, StdError, StdResult};

// use crate::error::ContractError;
use crate::msg::{IssuedTokensResponse, QueryMsg};
use crate::state::VTOKENS;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IssuedTokens { address, token_id } => {
            to_binary(&query_issued_tokens(deps, env, address, token_id)?)
        }
    }
}

fn query_issued_tokens(
    deps: Deps,
    _env: Env,
    address: String,
    token_id: u64,
) -> StdResult<IssuedTokensResponse> {
    let owner = deps.api.addr_validate(&address[..])?;
    let token = VTOKENS.may_load(deps.storage, (owner, token_id))?;

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
