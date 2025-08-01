use near_workspaces::types::NearToken;
use serde_json::json;

const FIVE_NEAR: NearToken = NearToken::from_near(5);

#[tokio::test]
async fn test_escrow_src_contract() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let root = sandbox.root_account()?;
    let user_account = root.create_subaccount("user").transact().await?.unwrap();
    let contract_account = root.create_subaccount("escrow_src").initial_balance(FIVE_NEAR).transact().await?.unwrap();

    let contract = contract_account.deploy(&contract_wasm).await?.unwrap();

    // Initialize the contract
    let outcome = contract
        .call("new")
        .args_json(json!({
            "rescue_delay": 3600,
            "access_token": "access_token.testnet"
        }))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Test get_rescue_delay
    let rescue_delay_outcome = contract
        .view("get_rescue_delay")
        .args_json(json!({}))
        .await?;
    assert_eq!(rescue_delay_outcome.json::<u64>()?, 3600);

    Ok(())
}

#[tokio::test]
async fn test_escrow_dst_contract() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let root = sandbox.root_account()?;
    let user_account = root.create_subaccount("user").transact().await?.unwrap();
    let contract_account = root.create_subaccount("escrow_dst").initial_balance(FIVE_NEAR).transact().await?.unwrap();

    let contract = contract_account.deploy(&contract_wasm).await?.unwrap();

    // Initialize the contract
    let outcome = contract
        .call("new")
        .args_json(json!({
            "rescue_delay": 3600,
            "access_token": "access_token.testnet"
        }))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Test get_rescue_delay
    let rescue_delay_outcome = contract
        .view("get_rescue_delay")
        .args_json(json!({}))
        .await?;
    assert_eq!(rescue_delay_outcome.json::<u64>()?, 3600);

    Ok(())
}

#[tokio::test]
async fn test_escrow_factory_contract() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let root = sandbox.root_account()?;
    let user_account = root.create_subaccount("user").transact().await?.unwrap();
    let contract_account = root.create_subaccount("factory").initial_balance(FIVE_NEAR).transact().await?.unwrap();

    let contract = contract_account.deploy(&contract_wasm).await?.unwrap();

    // Initialize the contract
    let outcome = contract
        .call("new")
        .args_json(json!({
            "escrow_src_implementation": "escrow_src.testnet",
            "escrow_dst_implementation": "escrow_dst.testnet",
            "proxy_src_bytecode_hash": [0u8; 32],
            "proxy_dst_bytecode_hash": [0u8; 32]
        }))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Test getter methods
    let src_impl_outcome = contract
        .view("get_escrow_src_implementation")
        .args_json(json!({}))
        .await?;
    assert_eq!(src_impl_outcome.json::<String>()?, "escrow_src.testnet");

    let dst_impl_outcome = contract
        .view("get_escrow_dst_implementation")
        .args_json(json!({}))
        .await?;
    assert_eq!(dst_impl_outcome.json::<String>()?, "escrow_dst.testnet");

    Ok(())
}

#[tokio::test]
async fn test_base_escrow_contract() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let root = sandbox.root_account()?;
    let user_account = root.create_subaccount("user").transact().await?.unwrap();
    let contract_account = root.create_subaccount("base_escrow").initial_balance(FIVE_NEAR).transact().await?.unwrap();

    let contract = contract_account.deploy(&contract_wasm).await?.unwrap();

    // Initialize the contract
    let outcome = contract
        .call("new")
        .args_json(json!({
            "rescue_delay": 3600,
            "access_token": "access_token.testnet"
        }))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Test getter methods
    let rescue_delay_outcome = contract
        .view("get_rescue_delay")
        .args_json(json!({}))
        .await?;
    assert_eq!(rescue_delay_outcome.json::<u64>()?, 3600);

    let factory_outcome = contract
        .view("get_factory")
        .args_json(json!({}))
        .await?;
    assert_eq!(factory_outcome.json::<String>()?, "base_escrow.testnet");

    Ok(())
} 