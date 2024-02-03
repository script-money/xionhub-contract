use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateHub {
        hub_name: String,
        need_pay: Coin,
    },
    SubscribeHub {
        hub_addr: Addr,
    },
    CreatePost {
        post_id: String,
        title: String,
        content: String,
    },
    LikePost {
        post_id: String,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Hub {
        creator: Addr,
    },
    UserSubscriptions {
        user: Addr,
        page: u64,
        size: u64,
    },
    HubAddresses {
        page: u64,
        size: u64,
    },
    HubPosts {
        user_addr: Addr,
        hub_addr: Addr,
        page: u64,
        size: u64,
    },
    PostLikes {
        post_id: String,
    },
}
