use cosmwasm_std::{coin, Addr};
use cw_multi_test::App;

use crate::state::Hub;

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
        .create_hub(&mut app, &creator, "Test Channel", coin(0, XION), &[])
        .unwrap();

    let resp = contract.query_hub(&app, &creator).unwrap();
    assert_eq!(
        resp,
        Hub {
            creator: creator.to_string(),
            name: "Test Channel".to_string(),
            payment: coin(0, XION),
            subscribers: vec![]
        }
    );
}
