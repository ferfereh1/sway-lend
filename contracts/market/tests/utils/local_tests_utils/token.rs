use crate::utils::number_utils::parse_units;
use fuels::{
    prelude::{abigen, Contract, StorageConfiguration, TxParameters},
    signers::WalletUnlocked,
    tx::{Address, Salt},
};
use rand::prelude::Rng;

use super::DeployTokenConfig;

abigen!(
    TokenContract,
    "tests/artefacts/token/token_contract-abi.json"
);

pub mod token_abi_calls {

    use super::*;
    use fuels::contract::call_response::FuelCallResponse;

    pub async fn mint(c: &TokenContract) -> FuelCallResponse<()> {
        let res = c.methods().mint().append_variable_outputs(1).call().await;
        res.unwrap()
    }
    pub async fn mint_and_transfer(
        c: &TokenContract,
        amount: u64,
        recipient: Address,
    ) -> FuelCallResponse<()> {
        let res = c
            .methods()
            .mint_and_transfer(amount, recipient)
            .append_variable_outputs(1);

        res.call().await.unwrap()
    }
    pub async fn initialize(
        c: &TokenContract,
        config: TokenInitializeConfig,
        mint_amount: u64,
        address: Address,
    ) -> FuelCallResponse<()> {
        c.methods()
            .initialize(config, mint_amount, address)
            .call()
            .await
            .expect("❌ Cannot initialize token")
    }
}

pub async fn get_token_contract_instance(
    wallet: &WalletUnlocked,
    deploy_config: &DeployTokenConfig,
) -> TokenContract {
    let mut name = deploy_config.name.clone();
    let mut symbol = deploy_config.symbol.clone();
    let decimals = deploy_config.decimals;

    let mut rng = rand::thread_rng();
    let salt = rng.gen::<[u8; 32]>();

    let id = Contract::deploy_with_parameters(
        "./tests/artefacts/token/token_contract.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::default(),
        Salt::from(salt),
    )
    .await
    .unwrap();

    let instance = TokenContract::new(id, wallet.clone());

    let mint_amount = parse_units(deploy_config.mint_amount, decimals);
    name.push_str(" ".repeat(32 - deploy_config.name.len()).as_str());
    symbol.push_str(" ".repeat(8 - deploy_config.symbol.len()).as_str());

    let config: TokenInitializeConfig = TokenInitializeConfig {
        name: fuels::core::types::SizedAsciiString::<32>::new(name).unwrap(),
        symbol: fuels::core::types::SizedAsciiString::<8>::new(symbol).unwrap(),
        decimals,
    };

    let address = Address::from(wallet.address());
    token_abi_calls::initialize(&instance, config, mint_amount, address).await;
    token_abi_calls::mint(&instance).await;

    instance
}
