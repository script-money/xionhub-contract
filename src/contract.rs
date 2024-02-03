use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

pub fn instantiate(_deps: DepsMut, _info: MessageInfo) -> StdResult<Response> {
    Ok(Response::new())
}

pub mod exec {
    use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response};

    use crate::{
        error::ContractError,
        state::{Hub, Post, HUBS, HUB_ADDRESS, LIKES, SUBSCRIPTIONS},
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
            posts: vec![],
        };
        HUBS.save(deps.storage, &sender_addr_str, &new_hub)?;

        let mut hub_addresses = HUB_ADDRESS.load(deps.storage).unwrap_or_default();
        hub_addresses.insert(0, sender_addr_str); // Insert the new address at the beginning of the vector
        HUB_ADDRESS.save(deps.storage, &hub_addresses)?;

        Ok(Response::new().add_attribute("method", "create_hub"))
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

        Ok(Response::new().add_attribute("method", "subscribe_to_hub"))
    }

    pub fn create_post(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        post_id: String,
        title: String,
        content: String,
    ) -> Result<Response, ContractError> {
        let hub_id = info.sender.as_str();

        // Check if the hub exists before proceeding
        if !HUBS.has(deps.storage, hub_id) {
            return Err(ContractError::HubNotFound {});
        }

        let mut hub: Hub = HUBS.load(deps.storage, hub_id)?;

        let post = Post {
            id: post_id.clone(),
            title,
            content,
            updated: env.block.time.seconds(),
        };

        hub.posts.insert(0, post);

        HUBS.save(deps.storage, hub_id, &hub)?;
        LIKES.save(deps.storage, &post_id, &0u64)?;

        Ok(Response::new().add_attribute("method", "create_post"))
    }

    pub fn like_post(
        deps: DepsMut,
        _info: MessageInfo,
        post_id: String,
    ) -> Result<Response, ContractError> {
        let mut likes = LIKES.load(deps.storage, &post_id)?;

        likes += 1;

        LIKES.save(deps.storage, &post_id, &likes)?;
        Ok(Response::new().add_attribute("method", "like_post"))
    }
}

pub mod query {
    use crate::state::{Hub, Post, HUBS, HUB_ADDRESS, LIKES, SUBSCRIPTIONS};
    use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Order, StdResult};

    pub fn query_hub(deps: Deps, creator: Addr) -> StdResult<Binary> {
        let hub = HUBS.load(deps.storage, &creator.as_str())?;
        to_json_binary(&hub)
    }

    pub fn query_user_subscriptions(
        deps: Deps,
        user_addr: Addr,
        page: u64,
        size: u64,
    ) -> StdResult<Binary> {
        let subscriptions: Vec<String> = SUBSCRIPTIONS
            .prefix(&user_addr)
            .keys(deps.storage, None, None, Order::Ascending)
            .filter_map(|result| result.ok())
            .collect();

        // Adjusted pagination logic to handle page 1 as the first page
        let start = page.saturating_sub(1).saturating_mul(size) as usize;
        let end = start.saturating_add(size as usize);
        // Query HUB_NAMES and HUBS based on subscriptions and pagination
        let hubs_info: Vec<Hub> = subscriptions[start..end.min(subscriptions.len())]
            .iter()
            .filter_map(|hub_name| HUBS.load(deps.storage, hub_name).ok())
            .collect();

        // Serialize the query result
        to_json_binary(&hubs_info.iter().map(|hub| &hub.name).collect::<Vec<_>>())
    }

    pub fn query_hub_addresses(deps: Deps, page: u64, size: u64) -> StdResult<Binary> {
        let hub_addresses = match HUB_ADDRESS.may_load(deps.storage)? {
            Some(addresses) => addresses,
            None => return to_json_binary(&Vec::<String>::new()), // Return an empty vector if HUB_ADDRESS is empty
        };
        let start = page.saturating_sub(1).saturating_mul(size) as usize;
        let end = std::cmp::min(start + size as usize, hub_addresses.len());
        let paged_hub_addresses: Vec<String> = hub_addresses
            .iter()
            .skip(start)
            .take(end - start)
            .cloned()
            .collect();

        to_json_binary(&paged_hub_addresses)
    }

    pub fn query_hub_posts(
        deps: Deps,
        user_addr: Addr,
        hub_addr: String,
        page: u64,
        size: u64,
    ) -> StdResult<Binary> {
        // Check if the user is subscribed to the hub
        let is_subscribed = SUBSCRIPTIONS
            .load(deps.storage, (&user_addr, &hub_addr))
            .unwrap_or(false);

        let hub = match HUBS.may_load(deps.storage, &hub_addr) {
            Ok(Some(hub)) => hub,
            _ => return to_json_binary(&Vec::<Post>::new()), // Return empty Vec if hub not found or error occurs
        };

        let posts = if is_subscribed {
            // If subscribed, paginate normally
            let start = page.saturating_sub(1).saturating_mul(size) as usize;
            let end = start.saturating_add(size as usize).min(hub.posts.len());

            if start >= hub.posts.len() {
                Vec::new()
            } else {
                hub.posts[start..end].to_vec()
            }
        } else {
            // If not subscribed, return only the latest post
            hub.posts.iter().take(1).cloned().collect()
        };

        to_json_binary(&posts)
    }

    pub fn query_post_likes(deps: Deps, post_id: String) -> StdResult<Binary> {
        let likes = LIKES.load(deps.storage, &post_id)?;
        to_json_binary(&likes)
    }
}
