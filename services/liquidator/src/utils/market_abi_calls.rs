pub mod market_abi_calls {

    use fuels::{
        prelude::{SettableContract, TxParameters},
        programs::call_response::FuelCallResponse,
        types::Address,
    };

    use crate::MarketContract;

    pub async fn absorb(
        market: &MarketContract,
        contract_ids: &[&dyn SettableContract],
        addresses: Vec<Address>,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        market
            .methods()
            .absorb(addresses)
            .set_contracts(contract_ids)
            .tx_params(TxParameters::default().set_gas_price(1))
            .call()
            .await
    }

    pub async fn is_liquidatable(
        market: &MarketContract,
        contract_ids: &[&dyn SettableContract],
        address: Address,
    ) -> Result<FuelCallResponse<bool>, fuels::types::errors::Error> {
        let tx_params = TxParameters::default().set_gas_price(1);
        market
            .methods()
            .is_liquidatable(address)
            .set_contracts(contract_ids)
            .tx_params(tx_params)
            // .estimate_tx_dependencies(None).await.unwrap()
            .simulate()
            .await
    }
}
