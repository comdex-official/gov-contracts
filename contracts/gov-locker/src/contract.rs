// !-----ONLY USE DURING TESTING, NOT OTHERWISE-----!
// #![allow(unused_imports, unused_variables, dead_code)]

#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, BankMsg, Coin, Decimal, DepsMut, Env, MessageInfo, Response, Storage, Timestamp,
    Uint128,
};

use cw2::set_contract_version;
use std::env;
use std::ops::{AddAssign, Sub, SubAssign};

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
        // ExecuteMsg::Unlock { app_id, denom } => {
        //     handle_unlock_nft(deps, &env, msg, info, app_id, denom)
        // }
        // ExecuteMsg::Withdraw {
        //     app_id,
        //     denom,
        //     amount,
        // } => withdraw(deps, &env, info, denom, amount),
        _ => Err(ContractError::CustomError {
            val: String::from("Not implemented"),
        }),
    }
}

/// Lock the sent tokens and create corresponding vtokens
pub fn handle_lock_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    app_id: u64,
    locking_period: LockingPeriod,
) -> Result<Response, ContractError> {
    // Only allow a single denomination
    if info.funds.is_empty() {
        return Err(ContractError::InsufficientFunds { funds: 0 });
    } else if info.funds.len() > 1 {
        return Err(ContractError::CustomError {
            val: String::from("Multiple denominations are not supported as yet."),
        });
    }

    if info.funds[0].amount.is_zero() {
        return Err(ContractError::InsufficientFunds { funds: 0 });
    }

    let mut state = STATE.load(deps.storage)?;

    // Load the locking period and weight
    let PeriodWeight { period, weight } = get_period(state.clone(), locking_period.clone())?;

    // Loads the NFT if present else None
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
                let new_vtoken = create_vtoken(
                    deps.storage,
                    &env,
                    &info,
                    locking_period.clone(),
                    period,
                    weight,
                )?;
                token.vtokens.push(new_vtoken);
                TOKENS.save(deps.storage, info.sender.clone(), &token)?;
            } else {
                // !------- BUG -------!
                // !------- Need to check if the tokens are Locked only then proceed -------!

                let mut vtoken = res[0].to_owned();

                if let Status::Locked = vtoken.status {
                    ()
                } else {
                    return Err(ContractError::NotLocked {});
                }

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

                TOKENS.save(deps.storage, info.sender.clone(), &token)?;
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

            let new_vtoken = create_vtoken(
                deps.storage,
                &env,
                &info,
                locking_period.clone(),
                period,
                weight,
            )?;
            VTOKENS.save(
                deps.storage,
                (info.sender.clone(), &info.funds[0].denom),
                &new_vtoken,
            )?;

            new_nft.vtokens.push(new_vtoken);
            TOKENS.save(deps.storage, info.sender.clone(), &new_nft)?;
        }
    }

    Ok(Response::new()
        .add_attribute("action", "lock")
        .add_attribute("from", info.sender))
}

fn create_vtoken(
    storage: &mut dyn Storage,
    env: &Env,
    info: &MessageInfo,
    locking_period: LockingPeriod,
    period: u64,
    weight: Decimal,
) -> Result<Vtoken, ContractError> {
    // Create the vtoken
    let mut vdenom = String::from("v");
    vdenom.push_str(&info.funds[0].denom[..]);

    let amount = weight * info.funds[0].amount;

    update_denom_supply(storage, vdenom.as_str(), amount.u128())?;

    Ok(Vtoken {
        token: info.funds[0].clone(),
        vtoken: Coin {
            denom: vdenom,
            amount,
        },
        period: locking_period,
        start_time: env.block.time,
        end_time: env.block.time.plus_seconds(period),
        status: Status::Locked,
    })
}

fn update_denom_supply(
    storage: &mut dyn Storage,
    vdenom: &str,
    quantity: u128,
) -> Result<(), ContractError> {
    let mut quantity = quantity;
    let vdenom_supply = SUPPLY.may_load(storage, vdenom)?;

    if let Some(val) = vdenom_supply {
        quantity = quantity + val;
    };

    SUPPLY.save(storage, vdenom, &quantity)?;

    Ok(())
}

fn update_locked(
    storage: &mut dyn Storage,
    vdenom: &str,
    quantity: u128,
) -> Result<(), ContractError> {
    // Update the Locked token count
    Ok(())
}

pub fn handle_unlock_nft(
    deps: DepsMut,
    env: &Env,
    msg: ExecuteMsg,
    info: MessageInfo,
    app_id: u64,
    denom: String,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    let mut Vtoken = VTOKENS.load(deps.storage, (info.sender, &denom)).unwrap();

    if Vtoken.status == Status::Unlocked {
        ContractError::AllreadyUnLocked {};
    }
    let t = Timestamp::from_seconds(state.unlock_period).seconds();

    if Vtoken.end_time < env.block.time
        && Vtoken.end_time.seconds() + state.unlock_period > env.block.time.seconds()
    {
        Vtoken.status = Status::Unlocking;
        // UNLOCKING.save(deps.storage, info.sender, data)
    } else if Vtoken.end_time.seconds() + state.unlock_period < env.block.time.seconds() {
        Vtoken.status = Status::Unlocked
    } else {
        ContractError::TimeNotOvered {};
    }

    Ok(Response::new()
        .add_attribute("action", "unlock"))
}

pub fn withdraw(
    deps: DepsMut,
    env: &Env,
    info: MessageInfo,
    denom: String,
    amount: u64,
) -> Result<Response, ContractError> {
    let mut Vtoken = VTOKENS.load(deps.storage, (info.sender.clone(), &denom)).unwrap();

    if Vtoken.status != Status::Unlocked {
        ContractError::NotUnlocked {};
    }

    if Vtoken.token.amount < Uint128::from(amount) {
        ContractError::InsufficientFunds {
            funds: Vtoken.token.amount.u128(),
        };
    }

    let withdraw_amount = Vtoken.token.amount.sub(Uint128::from(amount));
    Vtoken.token.amount -=Uint128::from(amount);
    VTOKENS.save(deps.storage, (info.sender.clone(),&info.funds[0].denom), &Vtoken)?;
    let vtoken=VTOKENS.load(deps.storage, (info.sender.clone(),&info.funds[0].denom))?;
    if vtoken.token.amount.is_zero() {
        VTOKENS.remove(deps.storage, (info.sender.clone(), &denom));
    }

    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin{
                denom,
                amount: withdraw_amount }],
        })
        .add_attribute("action", "Withdraw")
        .add_attribute("Recipent", info.sender))
}



fn get_period(state: State, locking_period: LockingPeriod) -> Result<PeriodWeight, ContractError> {
    Ok(match locking_period {
        LockingPeriod::T1 => state.t1,
        LockingPeriod::T2 => state.t2,
        LockingPeriod::T3 => state.t3,
        LockingPeriod::T4 => state.t4,
    })
}

#[cfg(test)]
mod tests {
    use std::io::Stderr;

    use crate::contract::{withdraw, self};
    use crate::msg::{QueryMsg, GetUnlockedTokenRespose};
    use crate::query::get_unlocked_tokens;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Addr, StdError, Deps};

    const DENOM: &str = "TKN";

    /// Returns default InstantiateMsg with each value in seconds.
    /// - t1 is 1 week (7*24*60*60), similarly, t2 is 2 weeks, t3 is 3 weeks
    /// and t4 is 4 weeks.
    /// - unlock_period is 1 week
    fn init_msg() -> InstantiateMsg {
        InstantiateMsg {
            t1: PeriodWeight {
                period: 604_800,
                weight: Decimal::from_atomics(Uint128::new(25), 2).unwrap(),
            },
            t2: PeriodWeight {
                period: 1_209_600,
                weight: Decimal::from_atomics(Uint128::new(50), 2).unwrap(),
            },
            t3: PeriodWeight {
                period: 1_814_400,
                weight: Decimal::from_atomics(Uint128::new(75), 2).unwrap(),
            },
            t4: PeriodWeight {
                period: 2_419_200,
                weight: Decimal::from_atomics(Uint128::new(100), 2).unwrap(),
            },
            unlock_period: 604_800,
        }
    }

    #[test]
    fn proper_initialization() {
        let env = mock_env();
        let mut deps = mock_dependencies();
        let info = mock_info("sender", &coins(0, DENOM.to_string()));

        let msg = init_msg();
        assert_eq!(msg.t1.weight.to_string(), "0.25");

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
        assert_eq!(res.messages.len(), 0);
        assert_eq!(res.attributes.len(), 2);

        let state = STATE.load(&deps.storage).unwrap();
        assert_eq!(state.t1, msg.t1);
        assert_eq!(state.t3, msg.t3);
    }

    #[test]
    fn lock() {
        // mock values
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &coins(0, DENOM.to_string()));

        let imsg = init_msg();
        instantiate(deps.as_mut(), env.clone(), info.clone(), imsg.clone()).unwrap();

        let msg = ExecuteMsg::Lock {
            app_id: 12,
            locking_period: LockingPeriod::T1,
        };

        // This should throw an error because the amount is zero
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap_err();
        match res {
            ContractError::InsufficientFunds { .. } => {}
            e => panic!("{:?}", e),
        };

        // Successful execution
        let info = mock_info("user1", &coins(100, DENOM.to_string()));

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
        assert_eq!(res.messages.len(), 0);
        assert_eq!(res.attributes.len(), 2);

        let sender_addr = Addr::unchecked("user1");
        let token = TOKENS.load(&deps.storage, sender_addr.clone()).unwrap();

        assert_eq!(token.owner, sender_addr.clone());
        assert_eq!(token.token_id, 1u64);
        assert_eq!(token.vtokens.len(), 1);
        // .token should be the same as locked tokens
        assert_eq!(
            token.vtokens[0].token,
            Coin {
                amount: Uint128::from(100u32),
                denom: DENOM.to_string()
            }
        );
        // .vtoken should be correct Vtoken released
        assert_eq!(
            token.vtokens[0].vtoken,
            Coin {
                amount: Uint128::from(25u32),
                denom: String::from("vTKN")
            }
        );
        assert_eq!(token.vtokens[0].start_time, env.block.time);
        assert_eq!(
            token.vtokens[0].end_time,
            env.block.time.plus_seconds(imsg.t1.period)
        );
        assert_eq!(token.vtokens[0].period, LockingPeriod::T1);
        assert_eq!(token.vtokens[0].status, Status::Locked);
    }

    #[test]
    fn test_withdraw() {

        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &coins(0, DENOM.to_string()));

        let imsg = init_msg();
        instantiate(deps.as_mut(), env.clone(), info.clone(), imsg.clone()).unwrap();

        let msg = ExecuteMsg::Lock {
            app_id: 12,
            locking_period: LockingPeriod::T1,
        };

        let info = mock_info("user1", &coins(100, DENOM.to_string()));

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();

        let mut vtoken =VTOKENS.load(&deps.storage, (info.sender.clone(),&info.funds[0].denom)).unwrap();
        vtoken.status= Status::Unlocked;

        assert_eq!(vtoken.token.denom,DENOM.to_string());
        assert_eq!(vtoken.status,Status::Unlocked);

        // Withdrawing 10 Tokens
        let err = withdraw(
            deps.as_mut(),
            &env, info.clone(),
            info.funds[0].denom.clone(),
            10
        );

        let mut _vtoken =VTOKENS.load(&deps.storage, (info.sender.clone(),&info.funds[0].denom)).unwrap();

        assert_eq!(err,Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![_vtoken.token],
        })
        .add_attribute("action", "Withdraw")
        .add_attribute("Recipent", info.sender.clone())));

        // Should left 100 - 10 = 90 tokens
        let mut _vtoken =VTOKENS.load(&deps.storage, (info.sender.clone(),&info.funds[0].denom)).unwrap();
        let n:u64=90;
        assert_eq!(_vtoken.token.amount,Uint128::from(n));

        // Withdrawing All Tokens and Should remove the vtoken.
        let err = withdraw(
            deps.as_mut(),
            &env, info.clone(),
            info.funds[0].denom.clone(),
            90
        );

        let mut _vtoken =VTOKENS.load(&deps.storage, (info.sender,&info.funds[0].denom));
        assert_eq!(_vtoken,Err(StdError::NotFound { kind: "gov_locker::state::Vtoken".to_string() }));
    }

    #[test]
    fn test_get_unlocked_tokens(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &coins(0, DENOM.to_string()));

        let imsg = init_msg();
        instantiate(deps.as_mut(), env.clone(), info.clone(), imsg.clone()).unwrap();

        let msg = ExecuteMsg::Lock {
            app_id: 12,
            locking_period: LockingPeriod::T1,
        };

        let info = mock_info("user1", &coins(100, DENOM.to_string()));

        let res = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            msg.clone()
        ).unwrap();

        let mut vtoken =VTOKENS.load(&deps.storage, (info.sender.clone(),&info.funds[0].denom)).unwrap();
        vtoken.status= Status::Unlocked;

        assert_eq!(vtoken.token.denom,DENOM.to_string());
        assert_eq!(vtoken.status,Status::Unlocked);

        let res =get_unlocked_tokens(
            deps.as_mut(),
            info.clone(),
            info.funds[0].denom.to_string()
        ).unwrap();
        let n:u64=100;
        // Should equal to 100 Tokens
        assert_eq!(res,GetUnlockedTokenRespose{ tokens: Uint128::from(n) })
    }
}
