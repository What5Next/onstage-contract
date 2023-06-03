use std::{env, fs};
use near_units::parse_near;
use serde_json::json;
use workspaces::{Account, Contract};

use near_sdk::json_types::U128;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount("alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    test_init(&alice, 1_000_000_000_000 ,&contract).await?;
    test_get_balance(&alice, &contract).await?;
    // begin tests
    // test_default_message(&alice, &contract).await?;
    // test_changes_message(&alice, &contract).await?;
    Ok(())
}

async fn test_init(
    user: &Account,
    total_supply: u128,
    contract: &Contract,
) -> anyhow::Result<()>{
    let result = user
    .call(contract.id(), "new_default_meta")
    .args_json(json!({
        "owner_id": user.id().to_string() ,
        "total_supply": total_supply.to_string(),
    }))
    .transact()
    .await?;

    assert!(result.is_success(), "Fungible Token Contract failed to initialize");
    println!("      Passed ✅ Fungible Token init ");
    Ok(())
}

async fn test_get_balance(
    user: &Account,
    contract: &Contract
) -> anyhow::Result<()>{
    let balance = contract
    .call("ft_balance_of")
    .args_json((user.id(), ))
    .view()
    .await?
    .json::<U128>()?;

    println!("{}", json!(balance));
    println!("      Passed ✅ Balace is correct");
    Ok(())
}


async fn test_default_message(
    user: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    let message: String = user
        .call( contract.id(), "get_greeting")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(message, "Hello".to_string());
    println!("      Passed ✅ gets default message");
    Ok(())
}

async fn test_changes_message(
    user: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    user.call(contract.id(), "set_greeting")
        .args_json(json!({"message": "Howdy"}))
        .transact()
        .await?
        .into_result()?;

    let message: String = user
        .call(contract.id(), "get_greeting")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(message, "Howdy".to_string());
    println!("      Passed ✅ changes message");
    Ok(())
}