use comdex_bindings::ComdexQuery;

pub fn setup() {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
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