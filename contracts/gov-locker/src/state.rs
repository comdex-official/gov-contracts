use crate::PeriodWeight;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub t1: PeriodWeight,
    pub t2: PeriodWeight,
    pub t3: PeriodWeight,
    pub t4: PeriodWeight,
    pub unlock_period: u128,
}

// pub struct Config {
//     state: Item<State>,
//     tokens: TokenInfo,
// }

pub const STATE: Item<State> = Item::new("state");
