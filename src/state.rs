use cosmwasm_std::Addr;
use cosmwasm_std::Coin;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Hub {
    pub creator: Addr,
    pub name: String,
    pub payment: Coin,
    pub subscribers: Vec<Addr>,
}

pub const HUBS: Map<&str, Hub> = Map::new("hubs");
pub const SUBSCRIPTIONS: Map<(&Addr, &str), bool> = Map::new("subscriptions");

pub const HUB_ADDRESS: Item<Vec<String>> = Item::new("hub_address");
