use cosmwasm_std::Addr;
use cosmwasm_std::Coin;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Hub {
    pub creator: String,
    pub name: String,
    pub payment: Coin,
    pub subscribers: Vec<Addr>,
}

pub const HUB: Map<&Addr, Hub> = Map::new("hub");
