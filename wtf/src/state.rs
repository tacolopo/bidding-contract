use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub admin: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const WINNING: Item<Uint128> = Item::new("highest_bid");
pub const DONATION_COUNT: Map<Addr, Uint128> = Map::new("donations");
pub const OPEN: Item<bool> = Item::new("contract_state");
