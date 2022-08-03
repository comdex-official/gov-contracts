#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Coin, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{LockingPeriod, PeriodWeight, State, Status, TokenInfo, Vtoken, STATE, TOKENS};

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
    let PeriodWeight { period, weight } = getPeriod(&state, locking_period)?;

    // Loads the NFT is present else None
    let nft = TOKENS.may_load(deps.storage, info.sender)?;

    match nft {
        Some(mut val) => {
            let res: Vec<Vtoken> = val
                .vtokens
                .into_iter()
                .filter(|s| s.token.denom == info.funds[0].denom && s.period == locking_period)
                .collect();

            if res.is_empty() {
                // create new token
                let new_vtoken = create_vtoken(&env, &info, locking_period, period, weight);
                val.vtokens.push(new_vtoken);
                TOKENS.save(deps.storage, info.sender, &val);
            } else {
                //------- What to do if the period already exists? -------
            }
        }
        None => {
            // Create a new NFT
            state.num_tokens += 1;

            let mut new_nft = TokenInfo {
                owner: info.sender,
                vtokens: vec![],
                token_id: state.num_tokens,
            };

            let new_vtoken = create_vtoken(&env, &info, locking_period, period, weight);

            new_nft.vtokens.push(new_vtoken);
            TOKENS.save(deps.storage, info.sender, &new_nft)?;
        }
    }

    if lockdata.status == Status::Locked && lockdata.status != Status::Released {
        let starttime = SystemTime::now();
        let _lcoked = LockedData {
            address,
            token_id: token_id.try_into().unwrap(),
            locking_Coin: lockingtoken,
            period: _type,
            start_time: starttime,
            end_time: starttime.add(_lockperiod.locking_period),
            status: Status::Locked,
        };
        LOCKDATA.save(deps.storage, address, &_lcoked);
        LDATA.insert(address, (token_id, _lcoked));
    } else {
        let start_time = SystemTime::now();
        let _lcoked = LockedData {
            address,
            token_id: token_id.try_into().unwrap(),
            locking_Coin: lockingtoken,
            period: _type,
            start_time: lockdata.start_time,
            end_time: lockdata.end_time.add(_lockperiod.locking_period),
            status: Status::Locked,
        };
        LOCKDATA.save(deps.storage, address, &_lcoked);
        LDATA.insert(address, (token_id, _lcoked));
    }
    Ok(Response::new())
}

fn create_vtoken(
    env: &Env,
    info: &MessageInfo,
    locking_period: LockingPeriod,
    period: u64,
    weight: f64,
) -> Vtoken {
    // Create the vtoken
    let mut vdenom = String::from("v");
    vdenom.push_str(&info.funds[0].denom[..]);

    // !------- Uint128 -> f64? -------!
    let amount = weight * info.funds[0].amount.into();

    Vtoken {
        token: info.funds[0],
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

pub fn getPeriod(
    state: &State,
    locking_period: LockingPeriod,
) -> Result<PeriodWeight, ContractError> {
    Ok(match locking_period {
        LockingPeriod::T1 => state.t1,
        LockingPeriod::T2 => state.t2,
        LockingPeriod::T3 => state.t3,
        LockingPeriod::T4 => state.t4,
    })
}

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
                weight: 0.25f64,
            },
            t2: PeriodWeight {
                period: 1_209_600,
                weight: 0.50f64,
            },
            t3: PeriodWeight {
                period: 1_814_400,
                weight: 0.75f64,
            },
            t4: PeriodWeight {
                period: 2_419_200,
                weight: 1.0f64,
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
        assert_eq!(
            msg.t1,
            PeriodWeight {
                period: 604_800,
                weight: 0.25
            }
        );
        assert_eq!(
            msg.t2,
            PeriodWeight {
                period: 1_209_600,
                weight: 0.50
            }
        );
        assert_eq!(
            msg.t3,
            PeriodWeight {
                period: 1_814_400,
                weight: 0.75
            }
        );
        assert_eq!(
            msg.t4,
            PeriodWeight {
                period: 2_419_200,
                weight: 1.00
            }
        );

        // let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        // assert_eq!(res.messages.len(), 0);
        // assert_eq!(res.attributes.len(), 2);
    }
}
