use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateHub { name: String, payment: Coin },
}

#[cw_serde]
pub enum QueryMsg {
    Hub { creator: Addr },
}
