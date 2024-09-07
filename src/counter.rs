use ethers::prelude::{abigen, Address, Http, Provider, U256};
use std::sync::Arc;
abigen!(CounterContract, "./src/counter.json");
use ethers::prelude::ContractError;

#[derive(Clone)]
pub struct Counter {
    pub client: Arc<Provider<Http>>,
    pub contract: CounterContract<Provider<Http>>,
}

impl Counter {
    pub fn new(address: Address, provider: Arc<Provider<Http>>) -> Self {
        let contract = CounterContract::new(address, provider.clone());
        Self {
            client: provider,
            contract,
        }
    }

    pub async fn get_number(&self) -> Result<U256, ContractError<Provider<Http>>> {
        Ok(self.contract.number().call().await?)
    }
}