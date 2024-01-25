use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CoreConstants {
    pub betax: i32,
    pub game_vault: Addr,
    pub odd: i32,
    pub odd2: i32,
}

pub const STATE: Item<State> = Item::new("state");

pub const CONSTCTX: Item<CoreConstants> = Item::new("const_ctx");
pub const BETAMOUNTS: Item<Vec<i16>> = Item::new("bet_amounts");
