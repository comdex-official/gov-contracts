#[allow(unused_variables)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::NotFound {
        kind: String::from("Not Implemented"),
    })
}
