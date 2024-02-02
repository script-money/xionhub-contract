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
    SubscribeToHub {
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
        page: usize,
        size: usize,
    },
    HubAddresses {
        page: usize,
        size: usize,
    },
    HubPosts {
        user_addr: Addr,
        hub_addr: Addr,
        page: usize,
        size: usize,
    },
    PostLikes {
        post_id: String,
    },
}
