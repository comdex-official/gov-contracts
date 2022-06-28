use std::marker::PhantomData;
use crate::msg::{ExecuteMsg, InstantiateMsg, ProposalResponseTotal, QueryMsg};
use comdex_bindings::{ComdexMessages, ComdexQuery};
use cosmwasm_std::{Deps, Response, Addr, Decimal, coins, OwnedDeps};
use cosmwasm_std::testing::{mock_env, MockApi, MockQuerier, MockStorage,MOCK_CONTRACT_ADDR, mock_info};
use cw_multi_test::{App, BankKeeper, Contract, ContractWrapper};
use cw_utils::{Threshold, Duration};
use crate::contract::{execute, instantiate, query, self};
use cosmwasm_vm::testing::{mock_instance};

fn mock_app() -> App {
    App::default()
}


// static WASM: &[u8] = include_bytes!("Governance.wasm");
static WASM: &[u8] = include_bytes!("governance.wasm");

#[test]
fn proper_initialization() {

    pub fn mock_dependencies1() -> OwnedDeps<MockStorage, MockApi, MockQuerier, ComdexQuery> {
        OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: MockQuerier::default(),
            custom_query_type: PhantomData,
        }
    }
    let mut deps = mock_dependencies1();

    // let deps = mock_instance(WASM, &[]);
    // assert_eq!(deps.required_features().len(), 0);

        const OWNER: &str = "admin001";
        let addr1 = Addr::unchecked("addr1");
        let addr2 = Addr::unchecked("addr2");

        let expected_msg1 = InstantiateMsg{
            threshold:Threshold::AbsoluteCount { weight: 3 },
            target:Duration::Height(3).to_string(),
        };

        let expected_msg2 = InstantiateMsg{
            threshold:Threshold::AbsolutePercentage { percentage: Decimal::percent(50) },
            target:Duration::Time(1000000).to_string(),
        };

        let expected_msg3 = InstantiateMsg{
            threshold:Threshold::ThresholdQuorum { 
                threshold: Decimal::percent(50),
                quorum: Decimal::percent(33) },
            target:Duration::Height(3).to_string(),
        };

        let info = mock_info(&OWNER, &coins(1000, "coin1"));

        let res1 = instantiate(deps.as_mut(), mock_env(), info.clone(), expected_msg1);
        let res2 = instantiate(deps.as_mut(), mock_env(), info.clone(), expected_msg2);
        let res3 = instantiate(deps.as_mut(), mock_env(), info.clone(), expected_msg3);
        

}




