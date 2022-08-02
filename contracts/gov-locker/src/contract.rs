use core::time;
use std::convert::TryInto;
use std::time::{SystemTime, Duration};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
// use cosmwasm_std::to_binary;
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, In};
use crate::state::{State, STATE, LockingPeriod, VPERIOD, VestingPeriod, Locked, Status, LOCKED};

// version info for migration info
const CONTRACT_NAME: &str = "gov-locker";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State {
        t1: msg.t1,
        t2: msg.t2,
        t3: msg.t3,
        t4: msg.t4,
        unlock_period: msg.unlock_period,
    };

    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("from", info.sender))
}

#[allow(unused_variables)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPriod { _in } => owner_set_vesting_period(deps, In),
        ExecuteMsg::LockTokens {
            token,
            value,
            _type,
        } => _lockNFT(deps, token, value, _type),
        _ => Err(ContractError::CustomError { val: String::from("Not implemented") }),
    }
}

pub fn owner_set_vesting_period(
    deps:DepsMut,
    msg:In
)->Result<Response, ContractError>{
    let n = msg._vperiods.len();
    for i in 0..n{
        let period = LockingPeriod{
            _type:msg._vperiods[i]._type,
            _time: msg._vperiods[i]._time,
            _weight:msg._vperiods[i]._weight,
        };
        VPERIOD.save(deps.storage, i.try_into().unwrap(), &period);
    }
    Ok(Response::new())
}

pub fn _lockNFT(
    deps: DepsMut,
    token:i64,
    value:i64,
    _type:i64
)->Result<Response, ContractError>{
    let _lockperiod = getPeriod(deps, _type);
    let _lcoked =Locked{
        token,
        value,
        period: _lockperiod._type,
        start_time:SystemTime::now(),
        end_time:,
        status:Status::Locked
    };

    LOCKED.save(deps.storage,&_lcoked);
    Ok(Response::new())
}


pub fn getPeriod(
    deps:DepsMut,
    vperiod:i64,
)->LockingPeriod{
    let _period = VPERIOD.load(deps.storage, vperiod).unwrap();
    return _period;
}

#[allow(unused_variables)]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::NotFound {
        kind: String::from("Not Implemented"),
    })
}
