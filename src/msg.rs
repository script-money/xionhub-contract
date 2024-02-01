use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateHub { hub_name: String, need_pay: Coin },
    SubscribeToHub { hub_addr: String },
}

#[cw_serde]
pub enum QueryMsg {
    Hub {
        creator: Addr,
    },
    UserSubscriptions {
        user: Addr,
        page: u32,
        page_size: u32,
    },
    HubAddresses {
        start_after: u32,
        limit: u32,
    },
}
