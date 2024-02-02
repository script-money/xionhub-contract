mod contract;
mod error;
mod msg;
mod state;

#[cfg(any(test, feature = "tests"))]
pub mod multitest;

use contract::{
    exec::{create_hub, create_post, like_post, subscribe_to_hub},
    query::{
        query_hub, query_hub_addresses, query_hub_posts, query_post_likes, query_user_subscriptions,
    },
};
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use error::ContractError;
use msg::{ExecuteMsg, QueryMsg};

use crate::msg::InstantiateMsg;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateHub { hub_name, need_pay } => {
            create_hub(deps, env, info, hub_name, need_pay)
        }
        ExecuteMsg::SubscribeToHub { hub_addr } => {
            subscribe_to_hub(deps, info, hub_addr.into_string())
        }
        ExecuteMsg::CreatePost {
            post_id,
            title,
            content,
        } => create_post(deps, env, info, post_id, title, content),
        ExecuteMsg::LikePost { post_id } => like_post(deps, info, post_id),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Hub { creator } => query_hub(deps, creator),
        QueryMsg::UserSubscriptions { user, page, size } => {
            query_user_subscriptions(deps, user, page, size)
        }
        QueryMsg::HubAddresses { page, size } => query_hub_addresses(deps, page, size),
        QueryMsg::HubPosts {
            user_addr,
            hub_addr,
            page,
            size,
        } => query_hub_posts(deps, user_addr, hub_addr.into_string(), page, size),
        QueryMsg::PostLikes { post_id } => query_post_likes(deps, post_id),
    }
}
