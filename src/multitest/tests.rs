use cosmwasm_std::{coin, coins, Addr};
use cw_multi_test::App;

use crate::{error::ContractError, state::Hub};

use super::contract::XionHubContract;

const XION: &str = "uxion";

#[test]
fn test_create_hub() {
    let owner = Addr::unchecked("owner");
    let creator = Addr::unchecked("creator");

    let mut app = App::default();

    let code_id = XionHubContract::store_code(&mut app);
    let contract =
        XionHubContract::instantiate(&mut app, code_id, &owner, "XionHub contract", None).unwrap();

    contract
        .create_hub(&mut app, &creator, "Test Channel", coin(0, XION))
        .unwrap();

    let resp = contract.query_hub(&app, &creator).unwrap();
    assert_eq!(
        resp,
        Hub {
            creator: creator.clone(),
            name: "Test Channel".to_string(),
            payment: coin(0, XION),
            subscribers: vec![]
        }
    );

    let resp2 = contract.query_hub_addresses(&app, 0, 10).unwrap();
    assert_eq!(resp2.len(), 1);
    assert_eq!(resp2, vec![creator.to_string()]);
}

#[test]
fn test_user_subscriptions() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");
    let creator = Addr::unchecked("creator");
    let user = Addr::unchecked("user");

    let code_id = XionHubContract::store_code(&mut app);
    let contract =
        XionHubContract::instantiate(&mut app, code_id, &owner, "XionHub contract", None).unwrap();

    let hub_name = "XionHub Office Channel";
    contract
        .create_hub(&mut app, &creator, hub_name, coin(0, XION))
        .unwrap();

    contract
        .subscribe_to_hub(&mut app, &user, creator.as_str(), &[])
        .unwrap();

    let page = 0;
    let page_size = 10;
    let resp = contract
        .query_user_subscriptions(&app, &user, page, page_size)
        .unwrap();

    assert_eq!(resp.len(), 1);
    assert_eq!(resp[0], hub_name);

    // test query_hub
    let resp = contract.query_hub(&app, &creator).unwrap();
    assert_eq!(
        resp,
        Hub {
            creator: creator,
            name: "XionHub Office Channel".to_string(),
            payment: coin(0, XION),
            subscribers: vec![user]
        }
    );
}

#[test]
fn test_user_subscriptions_with_token() {
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("user"), coins(100000, XION))
            .unwrap()
    });

    let owner = Addr::unchecked("owner");
    let creator = Addr::unchecked("creator");
    let user = Addr::unchecked("user"); // user has 100000 XION
    let user2 = Addr::unchecked("user2");

    let code_id = XionHubContract::store_code(&mut app);
    let contract =
        XionHubContract::instantiate(&mut app, code_id, &owner, "XionHub contract", None).unwrap();

    let hub_name = "VIP Channel";
    let funds = vec![coin(100000, XION)];
    contract
        .create_hub(&mut app, &creator, hub_name, coin(100000, XION))
        .unwrap();

    // Test InsufficientFunds when subscription fee is not met
    let err = contract
        .subscribe_to_hub(&mut app, &user2, creator.as_str(), &[])
        .unwrap_err();
    assert_eq!(ContractError::InsufficientFunds, err);

    let subscription_result = contract.subscribe_to_hub(&mut app, &user, creator.as_str(), &funds);
    assert!(subscription_result.is_ok());
}
