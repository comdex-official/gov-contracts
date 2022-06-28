use comdex_bindings::ComdexMessages;
use cosmwasm_std::{testing::{mock_info, mock_env}, Decimal, Uint128, Coin, Response, Addr, BankMsg};
use cw3::{Status, Vote};
use cw_storage_plus::Map;
use cw_utils::{Threshold, Expiration, Duration};

use crate::{msg::{InstantiateMsg, ExecuteMsg}, contract::{instantiate, execute_vote, execute, execute_execute, execute_deposit}, ContractError, state::{Proposal, Votes, next_id, PROPOSALS, APPPROPOSALS, AppProposalConfig}};


#[cfg(test)]
mod tests {
    use comdex_bindings::ComdexQuery;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{Addr, OwnedDeps};
    use cosmwasm_std::{BankMsg, Decimal};
    use cw_storage_plus::Map;
    use cw_utils::{Duration, Threshold};
    use std::marker::PhantomData;

    use super::*;

    const OWNER: &str = "admin0001";

    pub fn mock_dependencies1() -> OwnedDeps<MockStorage, MockApi, MockQuerier, ComdexQuery> {
        OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: MockQuerier::default(),
            custom_query_type: PhantomData,
        }
    }
}

#[test]
    fn test_gov() {
        tests::mock_dependencies1();
        const OWNER: &str = "admin001";
        let addr1 = Addr::unchecked("addr1");
        let addr2 = Addr::unchecked("addr2");
        let mut deps = tests::mock_dependencies1();
        let info = mock_info(OWNER, &[]);

        let instantiate_msg = InstantiateMsg {
            threshold: Threshold::ThresholdQuorum {
                threshold: Decimal::percent(50),
                quorum: Decimal::percent(33),
            },
            target: "0.0.0.0090".to_string(),
        };
        let res =instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        let ts = cosmwasm_std::Timestamp::from_nanos(1_655_745_339);
        let a = Uint128::from(123u128);
        let id = next_id(&mut deps.storage).unwrap();
        let mut prop = Proposal {
            title: "prop".to_string(),
            start_time: ts,
            description: "test prop".to_string(),
            start_height: 43,
            expires: Expiration::AtTime(cosmwasm_std::Timestamp::from_nanos(1_655_745_430)),
            msgs: vec![ComdexMessages::MsgWhitelistAppIdVaultInterest { app_mapping_id: id }],
            status: Status::Open,
            duration: Duration::Time(40),
            threshold: Threshold::ThresholdQuorum {
                threshold: Decimal::percent(50),
                quorum: Decimal::percent(33),
            },
            total_weight: 0,
            votes: Votes {
                yes: 0,
                no: 0,
                abstain: 0,
                veto: 0
            },
            deposit: vec![Coin {
                denom: "vote here".to_string(),
                amount: a,
            }],
            proposer: "validator201".to_string(),
            token_denom: "toVote".to_string(),
            min_deposit: 45,
            current_deposit: 56,
            app_mapping_id: id,
            is_slashed: true,
        };
        prop.update_status(&mock_env().block);
        PROPOSALS.save(&mut deps.storage, id, &prop);
        pub const VOTERDEPOSIT: Map<(u64, &Addr), Vec<Coin>> = Map::new("voter deposit");
        let info = mock_info(OWNER, &[]);
        let mut _vote = VOTERDEPOSIT.save(&mut deps.storage, (id, &info.sender), &info.funds);
        
        let _deposit_info = VOTERDEPOSIT
            .may_load(&deps.storage, (id, &info.sender))
            .unwrap();

        let deposit_msg = ExecuteMsg::Deposit { 
            proposal_id: id 
        };
        execute(deps.as_mut(), mock_env(), info.clone(),deposit_msg);
        // let err = execute_deposit(deps.as_mut(), mock_env(), info.clone(), id);

        let vote2 = ExecuteMsg::Vote {
            proposal_id: id,
            vote: Vote::Yes,
        };
        let info = mock_info(addr1.as_str(), &[]);
        execute(deps.as_mut(), mock_env(), info.clone(), vote2);


        let vote3 = ExecuteMsg::Vote {
            proposal_id: id,
            vote: Vote::Yes,
        };
        let info = mock_info(addr2.as_str(), &[]);
        execute(deps.as_mut(), mock_env(), info.clone(), vote3);


        prop.status =Status::Passed;
        PROPOSALS.save(&mut deps.storage, id, &prop);

        let execute_msg = ExecuteMsg::Execute { 
            proposal_id: id 
        };
        execute(deps.as_mut(), mock_env(), info.clone(),execute_msg);

        

        
        prop.status = Status::Passed;
        let a = Uint128::from(123u128);
        let deposit_info1 = Some(vec![Coin {
            denom: "coin".to_string(),
            amount: a,
        }])
        .unwrap();
        let mut _vot = VOTERDEPOSIT.save(&mut deps.storage, (id, &info.sender), &deposit_info1);
        _vot = PROPOSALS.save(&mut deps.storage, id, &prop);
        let refund_msg = ExecuteMsg::Refund { 
            proposal_id: id
        };
       let k = execute(deps.as_mut(), mock_env(), info.clone(),refund_msg);
        assert_eq!(
            k,
            Ok(Response::new()
                .add_message(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: deposit_info1,
                })
                .add_attribute("action", "refund")
                .add_attribute("sender", info.sender)
                .add_attribute("proposal_id", id.to_string()))
        );
    }