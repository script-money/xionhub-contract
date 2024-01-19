use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

pub fn instantiate(_deps: DepsMut, _info: MessageInfo) -> StdResult<Response> {
    Ok(Response::new())
}

pub mod exec {
    use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response};

    use crate::{
        error::ContractError,
        state::{Hub, HUB},
    };

    pub fn create_hub(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        name: String,
        payment: Coin,
    ) -> Result<Response, ContractError> {
        // Check if creator already has a hub
        if HUB.load(deps.storage, &info.sender).is_ok() {
            return Err(ContractError::CreatorAlreadyHasHub);
        }
        let new_hub = Hub {
            creator: info.sender.to_string(),
            name,
            payment,
            subscribers: vec![],
        };
        HUB.save(deps.storage, &info.sender, &new_hub)?;
        Ok(Response::default())
    }
}

pub mod query {
    use crate::state::HUB;
    use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, StdResult};

    pub fn query_hub(deps: Deps, creator: Addr) -> StdResult<Binary> {
        let hub = HUB.load(deps.storage, &creator)?;
        to_json_binary(&hub)
    }
}
