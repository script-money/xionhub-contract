use crate::{
    error::ContractError,
    execute, instantiate,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query,
    state::Hub,
};
use cosmwasm_std::{Addr, Coin, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

pub struct XionHubContract(Addr);

impl XionHubContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate<'a>(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        label: &str,
        admin: impl Into<Option<&'a Addr>>,
    ) -> StdResult<Self> {
        let admin = admin.into();

        app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {},
            &[],
            label,
            admin.map(Addr::to_string),
        )
        .map(XionHubContract)
        .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    pub fn create_hub(
        &self,
        app: &mut App,
        sender: &Addr,
        name: &str,
        payment: Coin,
        funds: &[Coin],
    ) -> Result<(), ContractError> {
        app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecuteMsg::CreateHub {
                name: name.to_string(),
                payment,
            },
            funds,
        )
        .map_err(|err| err.downcast().unwrap())
        .map(|_| ())
    }

    #[track_caller]
    pub fn query_hub(&self, app: &App, creator: &Addr) -> StdResult<Hub> {
        app.wrap().query_wasm_smart(
            self.0.clone(),
            &QueryMsg::Hub {
                creator: creator.clone(),
            },
        )
    }
}

impl From<XionHubContract> for Addr {
    fn from(contract: XionHubContract) -> Self {
        contract.0
    }
}
