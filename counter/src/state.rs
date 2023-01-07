use cosmwasm_std::{ Addr, Uint128 };
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin_address: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const COUNTER: Item<Uint128> = Item::new("counter");