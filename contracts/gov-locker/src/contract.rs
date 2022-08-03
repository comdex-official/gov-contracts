// #[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, In, InstantiateMsg};
use crate::state::{LockingPeriod, PeriodWeight, State, Status, TokenInfo, Vtoken, STATE, VTOKENS};

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
        // ExecuteMsg::SetPriod { _in } => owner_set_vesting_period(deps, In),
        // ExecuteMsg::LockTokens {
        //     token,
        //     value,
        //     _type,
        // } => _lockNFT(deps, token, value, _type),
        _ => Err(ContractError::CustomError {
            val: String::from("Not implemented"),
        }),
    }
}

// pub fn owner_set_vesting_period(
//     deps:DepsMut,
//     msg:In
// )->Result<Response, ContractError>{
//     let n = msg._vperiods.len();
//     for i in 0..n{
//         let period = LockingPeriod{
//             _type:msg._vperiods[i]._type,
//             _time: msg._vperiods[i]._time,
//             _weight:msg._vperiods[i]._weight,
//         };
//         VPERIOD.save(deps.storage, i.try_into().unwrap(), &period);
//     }
//     Ok(Response::new())
// }

// pub fn _lockNFT(
//     deps: DepsMut,
//     token:i64,
//     value:i64,
//     _type:i64
// )->Result<Response, ContractError>{
//     let _lockperiod = getPeriod(deps, _type);
//     let _lcoked =Locked{
//         token,
//         value,
//         period: _lockperiod._type,
//         start_time:SystemTime::now(),
//         end_time:,
//         status:Status::Locked
//     };

//     LOCKED.save(deps.storage,&_lcoked);
//     Ok(Response::new())
// }

// pub fn getPeriod(
//     deps:DepsMut,
//     vperiod:i64,
// )->LockingPeriod{
//     let _period = VPERIOD.load(deps.storage, vperiod).unwrap();
//     return _period;
// }

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    const DENOM: &str = "TKN";

    /// Returns default InstantiateMsg with each value in seconds.
    /// - t1 is 1 week (7*24*60*60), similarly,
    /// - t2 is 2 weeks
    /// - t3 is 3 weeks
    /// - t4 is 4 weeks
    /// - unlock_period is 1 week
    fn init_msg() -> InstantiateMsg {
        InstantiateMsg {
            t1: PeriodWeight {
                period: 604_800,
                weight: 0.25,
            },
            t2: PeriodWeight {
                period: 1_209_600,
                weight: 0.50,
            },
            t3: PeriodWeight {
                period: 1_814_400,
                weight: 0.75,
            },
            t4: PeriodWeight {
                period: 2_419_200,
                weight: 1.0,
            },
            unlock_period: 604_800,
        }
    }
    #[test]
    fn proper_initialization() {
        let env = mock_env();
        let mut deps = mock_dependencies();
        let info = mock_info("sender", &coins(0, &DENOM.to_string()));

        let msg = init_msg();

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg);
        // assert_eq!(res.messages.len(), 0);
        // assert_eq!(res.attributes.len(), 2);
    }
}
