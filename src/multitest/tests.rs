use cosmwasm_std::{coin, coins, Addr};
use cw_multi_test::App;
use uuid::Uuid;

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
            subscribers: vec![creator.clone()],
            posts: vec![]
        }
    );

    let resp2 = contract.query_hub_addresses(&app, 1, 10).unwrap();
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
        .subscribe_to_hub(&mut app, &user, &creator, &[])
        .unwrap();

    let page = 1;
    let size = 10;
    let resp = contract
        .query_user_subscriptions(&app, &user, page, size)
        .unwrap();

    assert_eq!(resp.len(), 1);
    assert_eq!(resp[0], hub_name);

    // test query_hub
    let resp = contract.query_hub(&app, &creator).unwrap();
    assert_eq!(
        resp,
        Hub {
            creator: creator.clone(),
            name: "XionHub Office Channel".to_string(),
            payment: coin(0, XION),
            subscribers: vec![creator.clone(), user],
            posts: vec![]
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

    assert_eq!(contract.query_user_has_hub(&app, &creator).unwrap(), true);
    assert_eq!(contract.query_user_has_hub(&app, &user2).unwrap(), false);

    // Test InsufficientFunds when subscription fee is not met
    let err = contract
        .subscribe_to_hub(&mut app, &user2, &creator, &[])
        .unwrap_err();
    assert_eq!(ContractError::InsufficientFunds, err);

    let subscription_result = contract.subscribe_to_hub(&mut app, &user, &creator, &funds);
    assert!(subscription_result.is_ok());
}

#[test]
fn test_create_and_query_post() {
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
        .create_post(
            &mut app,
            &creator,
            Uuid::new_v4(),
            "Hello World",
            "This is my first post",
        )
        .unwrap();

    let resp = contract
        .query_hub_posts(&app, &user, &creator, 1, 10)
        .unwrap();
    assert_eq!(resp.len(), 1);
    assert_eq!(resp[0].title, "Hello World");
    assert_eq!(resp[0].content, "This is my first post");

    contract
        .create_post(
            &mut app,
            &creator,
            Uuid::new_v4(),
            "A NEW ERA FOR BURNT",
            "Today, we end the silence with several major announcements.

            THE JOURNEY
            
            I originally started Burnt after the Banksy burning to create a better space for the future of art. Between 2021 and 2022 we focused on creating no-code products and early NFT standards for creators.
            
            In the process, we‘ve seen firsthand how the industry has been trapped in an echo chamber — building for ourselves and not in the interests of the world.
            
            While there are players genuinely trying to change this insular approach, its effects are still seen everywhere:
            
            From expecting users to learn our jargon, click on the ~correct~ links to somehow understand the difference between a legitimate project and a scam, we are still gatekeeping the very real benefits of Web3 behind impossible UX.
            
            And since almost all capital has been directed at the infrastructure level, innovation has been mostly in service of builders, not end users. To make matters worse, the industry has yet to demonstrate a meaningful Web3 use case to the world beyond what basically amounts to gambling.
            
            ANSWERING THE NEED FOR A NEW PARADIGM
            
            The result is we’ve built barriers instead of bridges — keeping real people out, while allowing us to feel superior in ivory towers.
            
            As an industry, we wanted to redefine ownership and distribute power to everyone, yet we became the elites we tried to build against.
            
            There is no redefining of ownership if we aren’t hitting critical needs nor providing adequate access to those who would benefit the most.
            
            I ask you, how can we redefine ownership
            if only the highly technical have access?
            
            It was this question that spawned a new era for Burnt.
            
            THE SECOND COMING OF BURNT
            
            First things first: As our new logo suggests, Burnt Finance has officially rebranded to Burnt. Our new name and visual direction signal our commitment to expanding the impact of Web3 beyond the confines of just DeFi.
            
            Along with these changes, the Burnt company vision has also evolved.
            
            Above all, Burnt exists to rebuild broken systems of ownership by any means necessary (whether it be technology, products, experiences, guerilla campaigns, fearless acts, etc.).
            
            To that end, we believe Web3 is fundamental in this pursuit due to its ability to re-balance the distribution of power, agency, and financial cooperation at scale.
            
            NEW BURNT, NEW BLOCKCHAIN
            
            
            Finally, after a year in the making, we are revealing XION, the first chain purpose-built for consumer adoption.
            
            The first of its kind, XION is intended to remove technical crypto barriers for consumers with a toolkit that includes seamless fiat on/off ramps, direct credit card purchases, familiar Web2 logins, safe account abstraction for mobile support, and zero gas fees.
            
            At its core, XION provides us the ability to shape the economics of our ecosystem from the ground up; aligning incentives across every level: from infrastructure, to developers, from products to creators and end-users.
            
            We believe everyone, regardless of technical knowledge, should be able to have the same access to true ownership. XION is our first step in realizing that future.
            
            WHAT TO EXPECT
            
            Over the next few weeks, we will be rolling out more information about XION, its roadmap to launch, and more soon-to-be revealed projects we have been building in stealth. Follow us on Twitter and Discord for the latest updates on the journey ahead.
            
            Here’s to the ashes we’ll create along the way.
            
            LOVE,
            Burnt Banksy",
        )
        .unwrap();

    let resp2 = contract
        .query_hub_posts(&app, &user, &creator, 1, 1)
        .unwrap();
    assert_eq!(resp2.len(), 1);
    assert_eq!(resp2[0].title, "A NEW ERA FOR BURNT");

    // if user is not subscribe hub, show latest post
    let resp3 = contract
        .query_hub_posts(&app, &user, &creator, 1, 10)
        .unwrap();
    assert_eq!(resp3.len(), 1);
    assert_eq!(resp3[0].title, "A NEW ERA FOR BURNT");

    // if user is subscribe hub, show all posts
    contract
        .subscribe_to_hub(&mut app, &user, &creator, &[])
        .unwrap();
    let resp4 = contract
        .query_hub_posts(&app, &user, &creator, 1, 10)
        .unwrap();
    assert_eq!(resp4.len(), 2);
}

#[test]
fn test_like_post() {
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

    let post_id = Uuid::new_v4();
    contract
        .create_post(
            &mut app,
            &creator,
            post_id,
            "Hello World",
            "This is my first post",
        )
        .unwrap();

    contract.like_post(&mut app, &user, &post_id).unwrap();

    let resp = contract.query_post_likes(&app, &post_id).unwrap();
    assert_eq!(resp, 1);

    let like_post_error = contract.like_post(&mut app, &user, &post_id).unwrap_err();
    assert_eq!(
        like_post_error,
        ContractError::PostAlreadyLiked {
            id: post_id.to_string()
        }
    );

    assert_eq!(
        contract
            .query_user_post_liked(&app, &user, &post_id)
            .unwrap(),
        true
    );
}

#[test]
fn test_query_zero_hubs() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");

    let code_id = XionHubContract::store_code(&mut app);
    let contract =
        XionHubContract::instantiate(&mut app, code_id, &owner, "XionHub contract", None).unwrap();

    let resp = contract.query_hub_addresses(&app, 1, 10).unwrap();
    assert_eq!(resp.len(), 0);
}

#[test]
fn test_query_hub_zero_posts() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");
    let creator = Addr::unchecked("creator");

    let code_id = XionHubContract::store_code(&mut app);
    let contract =
        XionHubContract::instantiate(&mut app, code_id, &owner, "XionHub contract", None).unwrap();

    let resp = contract
        .query_hub_posts(&app, &creator, &creator, 1, 10)
        .unwrap();
    assert_eq!(resp.len(), 0);
}

#[test]
fn test_query_user_zero_subscriptions() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");
    let user = Addr::unchecked("user");

    let code_id = XionHubContract::store_code(&mut app);
    let contract =
        XionHubContract::instantiate(&mut app, code_id, &owner, "XionHub contract", None).unwrap();

    let resp = contract
        .query_user_subscriptions(&app, &user, 1, 10)
        .unwrap();
    assert_eq!(resp.len(), 0);
}
