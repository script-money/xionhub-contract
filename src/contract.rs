use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

pub fn instantiate(_deps: DepsMut, _info: MessageInfo) -> StdResult<Response> {
    Ok(Response::new())
}

pub mod exec {
    use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response};

    use crate::{
        error::ContractError,
        state::{Hub, HUBS, SUBSCRIPTIONS},
    };

    pub fn create_hub(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        hub_name: String,
        need_pay: Coin,
    ) -> Result<Response, ContractError> {
        let sender_addr_str = info.sender.as_str().to_string();

        // Check if creator already has a hub
        if HUBS.load(deps.storage, &sender_addr_str).is_ok() {
            return Err(ContractError::CreatorAlreadyHasHub);
        }
        let new_hub = Hub {
            creator: info.sender.clone(),
            name: hub_name,
            payment: need_pay,
            subscribers: vec![],
        };
        HUBS.save(deps.storage, &sender_addr_str, &new_hub)?;

        Ok(Response::default())
    }

    pub fn subscribe_to_hub(
        deps: DepsMut,
        info: MessageInfo,
        hub_addr: String,
    ) -> Result<Response, ContractError> {
        // Check if the hub exists
        let hub = HUBS
            .load(deps.storage, &hub_addr)
            .map_err(|_| ContractError::HubNotFound)?;

        let sent_funds = info
            .funds
            .iter()
            .find(|coin| coin.denom == hub.payment.denom)
            .map(|coin| coin.amount)
            .unwrap_or_default();

        if !hub.payment.amount.is_zero() && sent_funds < hub.payment.amount {
            return Err(ContractError::InsufficientFunds);
        }

        // Check if the user is already subscribed
        let user_addr = info.sender;
        if SUBSCRIPTIONS
            .load(deps.storage, (&user_addr, &hub_addr))
            .is_ok()
        {
            return Err(ContractError::AlreadySubscribed);
        }

        // Subscribe the user to the hub
        SUBSCRIPTIONS.save(deps.storage, (&user_addr, &hub_addr), &true)?;

        // Optionally, add the user to the hub's subscribers list
        // This step depends on whether you want to maintain a list of subscribers in the Hub struct
        let mut hub = hub;
        hub.subscribers.push(user_addr.clone());
        HUBS.save(deps.storage, &hub_addr, &hub)?;

        Ok(Response::default())
    }
}

pub mod query {
    use crate::state::{Hub, HUBS, SUBSCRIPTIONS};
    use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Order, StdResult};

    pub fn query_hub(deps: Deps, creator: Addr) -> StdResult<Binary> {
        let hub = HUBS.load(deps.storage, &creator.as_str())?;
        to_json_binary(&hub)
    }

    pub fn query_user_subscriptions(
        deps: Deps,
        user_addr: Addr,
        page: u32,
        page_size: u32,
    ) -> StdResult<Binary> {
        let subscriptions: Vec<String> = SUBSCRIPTIONS
            .prefix(&user_addr)
            .keys(deps.storage, None, None, Order::Ascending)
            .filter_map(|result| result.ok())
            .collect();

        // Pagination logic
        let start = (page.saturating_sub(1) * page_size) as usize; // Corrected to handle underflow when page is 0
        let end = start.saturating_add(page_size as usize); // Use saturating_add to prevent potential overflow

        // Query HUB_NAMES and HUBS based on subscriptions and pagination
        let hubs_info: Vec<Hub> = subscriptions[start..end.min(subscriptions.len())]
            .iter()
            .filter_map(|hub_name| HUBS.load(deps.storage, hub_name).ok())
            .collect();

        // Serialize the query result
        to_json_binary(&hubs_info.iter().map(|hub| &hub.name).collect::<Vec<_>>())
    }
}
