mod contract;
mod error;
mod msg;
mod state;

#[cfg(any(test, feature = "tests"))]
pub mod multitest;

use contract::{exec::create_hub, query::query_hub};
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
        ExecuteMsg::CreateHub { name, payment } => create_hub(deps, env, info, name, payment),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Hub { creator } => query_hub(deps, creator),
    }
}
