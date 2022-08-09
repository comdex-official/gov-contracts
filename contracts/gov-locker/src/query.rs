// !------- IssuedVtokens query not implemented-------!
// !------- Tests for queries -------!

use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Coin, Deps, Env, MessageInfo, StdError, StdResult,
};

use crate::msg::{
    IssuedNftResponse, IssuedVtokensResponse, LockedTokensResponse, QueryMsg,
    UnlockedTokensResponse, UnlockingTokensResponse,
};
use crate::state::{LOCKED, TOKENS, UNLOCKED, UNLOCKING};

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
            kind: String::from("NFT not found"),
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
    let owner = if let Some(val) = address {
        deps.api.addr_validate(&val)?
    } else {
        info.sender
    };

    // result contains either a single token for the given denom or all
    // unlocked tokens for the given owner
    let tokens = UNLOCKED.may_load(deps.storage, owner)?;

    let mut unlocking_tokens = if let Some(val) = tokens {
        val
    } else {
        return Err(StdError::NotFound {
            kind: "No unlocked tokens".into(),
        });
    };

    unlocking_tokens = if let Some(val) = denom {
        unlocking_tokens
            .into_iter()
            .filter(|el| el.denom == val)
            .collect()
    } else {
        unlocking_tokens
    };

    Ok(UnlockedTokensResponse {
        tokens: unlocking_tokens,
    })
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
    let tokens = LOCKED.may_load(deps.storage, owner)?;

    let mut locked_tokens = if let Some(val) = tokens {
        val
    } else {
        return Err(StdError::NotFound {
            kind: "No locked tokens".into(),
        });
    };

    locked_tokens = if let Some(val) = denom {
        locked_tokens
            .into_iter()
            .filter(|el| el.denom == val)
            .collect()
    } else {
        locked_tokens
    };

    Ok(LockedTokensResponse {
        tokens: locked_tokens,
    })
}

pub fn query_unlocking_tokens(
    deps: Deps,
    _env: Env,
    info: MessageInfo,
    address: Option<String>,
    denom: Option<String>,
) -> StdResult<UnlockingTokensResponse> {
    // set `owner` for querying tokens
    let owner = if let Some(val) = address {
        deps.api.addr_validate(&val)?
    } else {
        info.sender
    };

    // result contains either a single token for the given denom or all
    // locked tokens for the given owner
    let mut tokens = UNLOCKING.may_load(deps.storage, owner)?;

    let mut unlocking_tokens = if let Some(val) = tokens {
        val
    } else {
        return Err(StdError::NotFound {
            kind: "No unlocking tokens".into(),
        });
    };

    unlocking_tokens = if let Some(val) = denom {
        unlocking_tokens
            .into_iter()
            .filter(|el| el.denom == val)
            .collect()
    } else {
        unlocking_tokens
    };

    Ok(UnlockingTokensResponse {
        tokens: unlocking_tokens,
    })
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{execute, instantiate};
    use crate::msg::{ExecuteMsg, InstantiateMsg};
    use crate::state::{LockingPeriod, PeriodWeight};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Decimal, Uint128};

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
    fn unlocked_tokens() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("owner", &coins(0, DENOM.to_string()));

        let owner = Addr::unchecked("owner");

        // Save some tokens in UNLOCKED
        let unlocked_tokens = vec![
            Coin {
                amount: Uint128::from(1000u128),
                denom: "DNM1".to_string(),
            },
            Coin {
                amount: Uint128::from(2000u128),
                denom: "DNM2".to_string(),
            },
        ];
        UNLOCKED
            .save(deps.as_mut().storage, owner.clone(), &unlocked_tokens)
            .unwrap();

        // Query unlocked tokens for specific denom
        let res = query_unlocked_tokens(
            deps.as_ref(),
            env.clone(),
            info.clone(),
            Some(owner.to_string()),
            Some("DNM1".to_string()),
        )
        .unwrap();

        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0].amount.u128(), 1000);
        assert_eq!(res.tokens[0].denom, "DNM1".to_string());

        // Query all tokens
        let res =
            query_unlocked_tokens(deps.as_ref(), env.clone(), info.clone(), None, None).unwrap();
        assert_eq!(res.tokens.len(), 2);
        assert_eq!(res.tokens[0].denom, "DNM1".to_string());
        assert_eq!(res.tokens[0].amount.u128(), 1000u128);
        assert_eq!(res.tokens[1].denom, "DNM2".to_string());
        assert_eq!(res.tokens[1].amount.u128(), 2000u128);
    }

    #[test]
    fn unlocking_tokens() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("owner", &coins(0, DENOM.to_string()));

        // Instantiate the contract
        // let imsg = init_msg();
        // instantiate(deps.as_mut(), env.clone(), info.clone(), imsg.clone()).unwrap();

        let owner = Addr::unchecked("owner");

        // Save some tokens in UNLOCKING
        let mut unlocking_tokens = vec![Coin {
            denom: DENOM.to_string(),
            amount: Uint128::from(100u32),
        }];
        UNLOCKING
            .save(deps.as_mut().storage, owner.clone(), &unlocking_tokens)
            .unwrap();

        let unlocking_coins = UNLOCKING
            .load(deps.as_ref().storage, owner.clone())
            .unwrap();
        assert_eq!(unlocking_coins.len(), 1);
        assert_eq!(unlocking_coins[0].amount.u128(), 100u128);
        assert_eq!(unlocking_coins[0].denom, DENOM.to_string());

        // Query the UNLOCKING map
        let res = query_unlocking_tokens(
            deps.as_ref(),
            env.clone(),
            info.clone(),
            Some(owner.to_string()),
            Some(DENOM.to_string()),
        )
        .unwrap();
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0].amount.u128(), 100u128);
        assert_eq!(res.tokens[0].denom, DENOM.to_string());

        // Save another token denom
        unlocking_tokens.push(Coin {
            denom: "DNM1".to_string(),
            amount: Uint128::from(100u32),
        });
        UNLOCKING
            .save(deps.as_mut().storage, owner.clone(), &unlocking_tokens)
            .unwrap();

        let unlocking_coins = UNLOCKING
            .load(deps.as_ref().storage, owner.clone())
            .unwrap();
        assert_eq!(unlocking_coins.len(), 2);
        assert_eq!(unlocking_coins[0].amount.u128(), 100u128);
        assert_eq!(unlocking_coins[0].denom, DENOM.to_string());
        assert_eq!(unlocking_coins[1].amount.u128(), 100u128);
        assert_eq!(unlocking_coins[1].denom, "DNM1".to_string());

        // Query with specific denom
        let res = query_unlocking_tokens(
            deps.as_ref(),
            env.clone(),
            info.clone(),
            Some(owner.to_string()),
            Some(DENOM.to_string()),
        )
        .unwrap();
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0].amount.u128(), 100u128);
        assert_eq!(res.tokens[0].denom, DENOM.to_string());

        // Query without a specific denom
        let res = query_unlocking_tokens(
            deps.as_ref(),
            env.clone(),
            info.clone(),
            Some(owner.to_string()),
            None,
        )
        .unwrap();
        assert_eq!(res.tokens.len(), 2);
        assert_eq!(res.tokens[0].amount.u128(), 100u128);
        assert_eq!(res.tokens[0].denom, DENOM.to_string());
        assert_eq!(res.tokens[1].amount.u128(), 100u128);
        assert_eq!(res.tokens[1].denom, "DNM1".to_string());
    }

    #[test]
    fn locked_tokens() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("owner", &coins(0, "DNM1".to_string()));

        let owner = Addr::unchecked("owner");

        let locked_tokens = coins(1000, "DNM1".to_string());
        LOCKED
            .save(deps.as_mut().storage, owner.clone(), &locked_tokens)
            .unwrap();

        let res = query_locked_tokens(
            deps.as_ref(),
            env.clone(),
            info.clone(),
            Some(owner.to_string()),
            Some("DNM1".into()),
        )
        .unwrap();
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0].amount.u128(), 1000);
        assert_eq!(res.tokens[0].denom, "DNM1".to_string());
    }
}
