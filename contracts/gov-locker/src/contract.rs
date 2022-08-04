#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Coin, Decimal, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use std::env;
use std::ops::AddAssign;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{
    LockingPeriod, PeriodWeight, State, Status, TokenInfo, Vtoken, LOCKED, STATE, SUPPLY, TOKENS,
    UNLOCKED, UNLOCKING, VTOKENS,
};

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
        num_tokens: 0,
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
        ExecuteMsg::Lock {
            app_id,
            locking_period,
        } => handle_lock_nft(deps, env, info, app_id, locking_period),

        ExecuteMsg::Unlock {
            app_id,
            amount,
            denom,
        } => handle_unlock_nft(deps, env, info, app_id, denom),

        _ => Err(ContractError::CustomError {
            val: String::from("Not implemented"),
        }),
    }
}

pub fn handle_lock_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    app_id: u64,
    locking_period: LockingPeriod,
) -> Result<Response, ContractError> {
    // Only allow a single denomination
    if info.funds.len() == 0 {
        return Err(ContractError::InsufficientFunds { funds: 0 });
    } else if info.funds.len() > 1 {
        return Err(ContractError::CustomError {
            val: String::from("Multiple denominations are not supported as yet."),
        });
    }

    let mut state = STATE.load(deps.storage)?;

    // Load the locking period and weight
    let PeriodWeight { period, weight } = get_period(state.clone(), locking_period.clone())?;

    // Loads the NFT is present else None
    let nft = TOKENS.may_load(deps.storage, info.sender.clone())?;

    match nft {
        Some(mut token) => {
            let res: Vec<&Vtoken> = token
                .vtokens
                .iter()
                .filter(|s| s.token.denom == info.funds[0].denom && s.period == locking_period)
                .collect();

            if res.is_empty() {
                // create new token
                let new_vtoken = create_vtoken(&env, &info, locking_period.clone(), period, weight);
                token.vtokens.push(new_vtoken);
                TOKENS.save(deps.storage, info.sender.clone(), &token);
            } else {
                let mut vtoken = res[0].to_owned();

                let mut remaining: Vec<Vtoken> = token
                    .vtokens
                    .into_iter()
                    .filter(|s| {
                        !(s.token.denom == info.funds[0].denom && s.period == locking_period)
                    })
                    .collect();

                // Increase the token count
                vtoken.token.amount.add_assign(info.funds[0].clone().amount);

                // Increase the vtoken count
                vtoken
                    .vtoken
                    .amount
                    .add_assign(weight * info.funds[0].amount);

                // The new start time will be current block time, i.e. the old
                // tokens will also unlock with the new tokens.
                vtoken.start_time = env.block.time;
                vtoken.end_time = env.block.time.plus_seconds(period);

                remaining.push(vtoken);
                token.vtokens = remaining;

                TOKENS.save(deps.storage, info.sender.clone(), &token);
            }
        }
        None => {
            // Create a new NFT
            state.num_tokens += 1;

            let mut new_nft = TokenInfo {
                owner: info.sender.clone(),
                vtokens: vec![],
                token_id: state.num_tokens,
            };

            let new_vtoken = create_vtoken(&env, &info, locking_period.clone(), period, weight);

            new_nft.vtokens.push(new_vtoken);
            TOKENS.save(deps.storage, info.sender.clone(), &new_nft)?;
        }
    }
    Ok(Response::new()
        .add_attribute("action", "lock")
        .add_attribute("from", info.sender))
}

fn create_vtoken(
    env: &Env,
    info: &MessageInfo,
    locking_period: LockingPeriod,
    period: u64,
    weight: Decimal,
) -> Vtoken {
    // Create the vtoken
    let mut vdenom = String::from("v");
    vdenom.push_str(&info.funds[0].denom[..]);

    let amount = weight * info.funds[0].amount;

    Vtoken {
        token: info.funds[0].clone(),
        vtoken: Coin {
            denom: vdenom,
            amount,
        },
        period: locking_period,
        start_time: env.block.time,
        end_time: env.block.time.plus_seconds(period),
        status: Status::Locked,
    }
}

pub fn handle_unlock_nft(
    deps: DepsMut,
    env: &Env,
    msg: ExecuteMsg,
    info: MessageInfo,
    app_id: u64,
    denom: String,
    tokenId: u64,
) -> Result<Response, ContractError> {
    let nft = TOKENS.may_load(deps.storage, info.sender).unwrap();
    let Vtoken = VTOKENS
        .load(deps.storage, (info.sender, tokenId, &denom))
        .unwrap();

    if Vtoken.end_time < env.block.time {
        Vtoken.status = Status::Unlocked
    } else {
        ContractError::TimeNotOvered {};
    }

    Ok(Response::new()
        .add_attribute("action", "unlock")
        .add_attribute("from", info.sender))
}

pub fn get_period(
    state: State,
    locking_period: LockingPeriod,
) -> Result<PeriodWeight, ContractError> {
    Ok(match locking_period {
        LockingPeriod::T1 => state.t1,
        LockingPeriod::T2 => state.t2,
        LockingPeriod::T3 => state.t3,
        LockingPeriod::T4 => state.t4,
    })
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::coins;
//     use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

//     const DENOM: &str = "TKN";

//     /// Returns default InstantiateMsg with each value in seconds.
//     /// - t1 is 1 week (7*24*60*60), similarly, t2 is 2 weeks, t3 is 3 weeks
//     /// and t4 is 4 weeks.
//     /// - unlock_period is 1 week
//     fn init_msg() -> InstantiateMsg {
//         InstantiateMsg {
//             t1: PeriodWeight {
//                 period: 604_800,
//                 weight: 0.25f64,
//             },
//             t2: PeriodWeight {
//                 period: 1_209_600,
//                 weight: 0.50f64,
//             },
//             t3: PeriodWeight {
//                 period: 1_814_400,
//                 weight: 0.75f64,
//             },
//             t4: PeriodWeight {
//                 period: 2_419_200,
//                 weight: 1.0f64,
//             },
//             unlock_period: 604_800,
//         }
//     }

//     #[test]
//     fn proper_initialization() {
//         let env = mock_env();
//         let mut deps = mock_dependencies();
//         let info = mock_info("sender", &coins(0, &DENOM.to_string()));

//         let msg = init_msg();

//         // let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
//         // assert_eq!(res.messages.len(), 0);
//         // assert_eq!(res.attributes.len(), 2);
//     }
// }
