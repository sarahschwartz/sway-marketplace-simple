use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(MyContract, "out/debug/marketplace-abi.json");

async fn get_contract_instance() -> (MyContract, ContractId, Vec<WalletUnlocked>) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(3),             /* Three wallets */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    let wallet = wallets.pop().unwrap();

    let id = Contract::deploy(
        "./out/debug/marketplace.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/marketplace-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet);

    (instance, id.into(), wallets)
}

#[tokio::test]
async fn can_get_contract_id() {
    let (instance, _id, wallets) = get_contract_instance().await;
    // Now you have an instance of your contract you can use to test each function

    // get access to some test wallets
    let wallet_1 = wallets.get(0).unwrap();
    let wallet_2 = wallets.get(1).unwrap();

    // item 1 params
    let item_1_metadata: SizedAsciiString<20> = "metadata__url__here_"
        .try_into()
        .expect("Should have succeeded");
    let item_1_price: u64 = 1533;

    // list item 1 from wallet_1
    let _item_1_result = instance
        .with_wallet(wallet_1.clone())
        .unwrap()
        .methods()
        .list_item(item_1_price, item_1_metadata)
        .call()
        .await
        .unwrap();

    // Bytes representation of the asset ID of the "base" asset used for gas fees.
    const BASE_ASSET_ID: AssetId = AssetId::new([0u8; 32]);

    // check the balances of wallet_1 and wallet_2
    let initial_balance_1: u64 = wallet_1.get_asset_balance(&BASE_ASSET_ID).await.unwrap();
    let initial_balance_2: u64 = wallet_2.get_asset_balance(&BASE_ASSET_ID).await.unwrap();

    // call params to send the project price in the buy_item fn
    let call_params = CallParameters::new(Some(item_1_price), Some(BASE_ASSET_ID), None);

    // buy item 1 from wallet_2
    let _item_1_purchase = instance
        .with_wallet(wallet_2.clone())
        .unwrap()
        .methods()
        .buy_item(1)
        .append_variable_outputs(1)
        .call_params(call_params)
        .call()
        .await
        .unwrap();

    // check the balances of wallet_1 and wallet_2
    let new_balance_1: u64 = wallet_1.get_asset_balance(&BASE_ASSET_ID).await.unwrap();
    let new_balance_2: u64 = wallet_2.get_asset_balance(&BASE_ASSET_ID).await.unwrap();

    // make sure the price was transferred from wallet_2 to wallet_1
    assert!(new_balance_1 == initial_balance_1 + item_1_price);
    assert!(new_balance_2 == initial_balance_2 - item_1_price);
}
