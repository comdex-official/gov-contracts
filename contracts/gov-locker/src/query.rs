use cosmwasm_std::{entry_point, to_binary, Binary, Deps, Env, StdError, StdResult, Addr, MessageInfo};

use crate::ContractError;
// use crate::error::ContractError;
use crate::msg::{IssuedTokensResponse, QueryMsg, GetUnlockedTokenRespose};
use crate::state::{TOKENS, VTOKENS, Status};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IssuedTokens{address,token_id}=>{to_binary(&query_issued_tokens(deps,env,address,token_id)?)}
        QueryMsg::GetUnlockedTokens {  denom } => get_unlocked_tokens(deps,info, denom)}}


fn query_issued_tokens(
    deps: Deps,
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
    deps: Deps,
    info:MessageInfo,
    denom: String,
)->StdResult<GetUnlockedTokenRespose>{

    let Vtoken = VTOKENS
    .load(deps.storage, (info.sender, &denom))
    .unwrap();

    if Vtoken.status != Status::Unlocked{
        ContractError::NotUnlocked {  };
    }

    let res = GetUnlockedTokenRespose{
        tokens: Vtoken.token.amount,
    };
    Ok(res)


}
