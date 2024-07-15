use std::sync::Arc;

use ethers::signers::LocalWallet;
use hyperliquid::{
    types::{
        exchange::response::{Response, StatusType},
        Chain,
    },
    Exchange, Hyperliquid,
};

#[tokio::main]
async fn main() {
    // Key was randomly generated for testing and shouldn't be used with any real funds
    let wallet: Arc<LocalWallet> = Arc::new(
        "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e"
            .parse()
            .unwrap(),
    );

    let exchange: Exchange = Hyperliquid::new(Chain::ArbitrumTestnet);

    println!("Creating subaccount...");
    let name = {
        let suffix = timestamp().to_string();

        // slice the last 4 characters
        let suffix = &suffix[suffix.len() - 4..];

        format!("Acc-{}", suffix)
    };

    let response = exchange
        .create_sub_account(wallet.clone(), name.clone())
        .await
        .expect("Failed to create subaccount");

    let response = match response {
        Response::Ok(sub_account) => sub_account,
        Response::Err(error) => {
            panic!("Failed to create subaccount: {:?}", error)
        }
    };

    let sub_account_user = match response.data {
        Some(StatusType::Address(address)) => address,
        _ => panic!("Failed to get subaccount address: {:?}", response),
    };

    println!(
        "Subaccount created with name {} and user address: {:x} ✓",
        name, sub_account_user
    );

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    let new_name = {
        let suffix = timestamp().to_string();

        // slice the last 4 characters
        let suffix = &suffix[suffix.len() - 4..];

        format!("Acc-{}", suffix)
    };

    println!("Renaming subaccount to: {}", new_name);

    let response = exchange
        .sub_account_modify(wallet.clone(), new_name, sub_account_user)
        .await
        .expect("Failed to rename subaccount");

    let response = match response {
        Response::Ok(sub_account) => sub_account,
        Response::Err(error) => {
            panic!("Failed to rename subaccount: {:?}", error)
        }
    };

    println!("Subaccount rename response: {:?} ✓", response);

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    println!("Depositing 1 USD to subaccount...");

    let usd = 1_000_000;

    let is_deposit = true;

    let response = exchange
        .sub_account_transfer(wallet.clone(), is_deposit, sub_account_user, usd)
        .await
        .expect("Failed to deposit funds");

    let response = match response {
        Response::Ok(response) => response,
        Response::Err(error) => {
            panic!("Failed to deposit funds: {:?}", error)
        }
    };

    println!("Deposit response: {:?} ✓", response);

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    println!("Withdrawing funds from subaccount...");

    let response = exchange
        .sub_account_transfer(wallet.clone(), !is_deposit, sub_account_user, usd)
        .await
        .expect("Failed to withdraw funds");

    let response = match response {
        Response::Ok(response) => response,
        Response::Err(error) => {
            panic!("Failed to withdraw funds: {:?}", error)
        }
    };

    println!("Withdraw response: {:?} ✓", response);
}

// timestamp in miliseconds using std::time::SystemTime
fn timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
