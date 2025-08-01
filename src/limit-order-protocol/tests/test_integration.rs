use near_workspaces::types::NearToken;
use serde_json::json;

const FIVE_NEAR: NearToken = NearToken::from_near(5);

#[tokio::test]
async fn test_limit_order_protocol_contract() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let root = sandbox.root_account()?;
    let user_account = root.create_subaccount("user").transact().await?.unwrap();
    let contract_account = root.create_subaccount("protocol").initial_balance(FIVE_NEAR).transact().await?.unwrap();

    let contract = contract_account.deploy(&contract_wasm).await?.unwrap();

    // Initialize the contract
    let outcome = contract
        .call("new")
        .args_json(json!({
            "domain_separator": [1u8; 32],
            "weth": "weth.testnet"
        }))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Test domain separator
    let domain_separator_outcome = contract
        .view("domain_separator")
        .args_json(json!({}))
        .await?;
    assert_eq!(domain_separator_outcome.json::<Vec<u8>>()?, vec![1u8; 32]);

    // Test is_paused
    let is_paused_outcome = contract
        .view("is_paused")
        .args_json(json!({}))
        .await?;
    assert_eq!(is_paused_outcome.json::<bool>()?, false);

    Ok(())
}

#[tokio::test]
async fn test_order_mixin_contract() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let root = sandbox.root_account()?;
    let user_account = root.create_subaccount("user").transact().await?.unwrap();
    let contract_account = root.create_subaccount("mixin").initial_balance(FIVE_NEAR).transact().await?.unwrap();

    let contract = contract_account.deploy(&contract_wasm).await?.unwrap();

    // Initialize the contract
    let outcome = contract
        .call("new")
        .args_json(json!({
            "domain_separator": [1u8; 32],
            "weth": "weth.testnet"
        }))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Test get_domain_separator
    let domain_separator_outcome = contract
        .view("get_domain_separator")
        .args_json(json!({}))
        .await?;
    assert_eq!(domain_separator_outcome.json::<Vec<u8>>()?, vec![1u8; 32]);

    // Test get_weth
    let weth_outcome = contract
        .view("get_weth")
        .args_json(json!({}))
        .await?;
    assert_eq!(weth_outcome.json::<String>()?, "weth.testnet");

    Ok(())
}

#[tokio::test]
async fn test_order_lib_contract() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let root = sandbox.root_account()?;
    let user_account = root.create_subaccount("user").transact().await?.unwrap();
    let contract_account = root.create_subaccount("lib").initial_balance(FIVE_NEAR).transact().await?.unwrap();

    let contract = contract_account.deploy(&contract_wasm).await?.unwrap();

    // Initialize the contract
    let outcome = contract
        .call("new")
        .args_json(json!({
            "domain_separator": [1u8; 32]
        }))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Test get_domain_separator
    let domain_separator_outcome = contract
        .view("get_domain_separator")
        .args_json(json!({}))
        .await?;
    assert_eq!(domain_separator_outcome.json::<Vec<u8>>()?, vec![1u8; 32]);

    Ok(())
} 