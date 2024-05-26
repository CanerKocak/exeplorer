use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::api::call::CallResult;
use serde::Serialize;

const TOKEN_CANISTER_ID: &str = "rh2pm-ryaaa-aaaan-qeniq-cai"; // Windoge98 Token

#[derive(CandidType, Deserialize)]
struct Account {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Serialize)]
struct Balance(Nat);

#[derive(CandidType, Deserialize, Serialize)]
struct TransactionRange {
    transactions: Vec<Transaction>,
}

#[derive(CandidType, Deserialize, Serialize)]
struct Transaction {
    burn: Option<Burn>,
    kind: String,
    mint: Option<Mint>,
    timestamp: u64,
    index: Nat,
    transfer: Option<Transfer>,
}

#[derive(CandidType, Deserialize, Serialize)]
struct Burn {
    from: AccountInfo,
    amount: Nat,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Serialize)]
struct Mint {
    to: AccountInfo,
    amount: Nat,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Serialize)]
struct Transfer {
    from: AccountInfo,
    to: AccountInfo,
    amount: Nat,
    fee: Option<Nat>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Serialize)]
struct AccountInfo {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[ic_cdk::update]
async fn icrc1_balance_of(account: Account) -> CallResult<(Balance,)> {
    let response: CallResult<(Balance,)> = ic_cdk::call(
        Principal::from_text(TOKEN_CANISTER_ID).unwrap(),
        "icrc1_balance_of",
        (account,),
    )
    .await;
    response
}

#[ic_cdk::update]
async fn icrc1_total_supply() -> CallResult<(Balance,)> {
    let response: CallResult<(Balance,)> = ic_cdk::call(
        Principal::from_text(TOKEN_CANISTER_ID).unwrap(),
        "icrc1_total_supply",
        (),
    )
    .await;
    response
}

#[ic_cdk::update]
async fn get_transactions(start: Nat, length: Nat) -> CallResult<(TransactionRange,)> {
    let response: CallResult<(TransactionRange,)> = ic_cdk::call(
        Principal::from_text(TOKEN_CANISTER_ID).unwrap(),
        "get_transactions",
        (start, length),
    )
    .await;
    response
}

#[ic_cdk::update]
async fn get_token_info() -> Result<(String, String, Nat, u8, Nat, Nat), String> {
    let name: CallResult<(String,)> = ic_cdk::call(
        Principal::from_text(TOKEN_CANISTER_ID).unwrap(),
        "icrc1_name",
        (),
    )
    .await;

    let symbol: CallResult<(String,)> = ic_cdk::call(
        Principal::from_text(TOKEN_CANISTER_ID).unwrap(),
        "icrc1_symbol",
        (),
    )
    .await;

    let total_supply: CallResult<(Balance,)> = icrc1_total_supply().await;
    let decimals: CallResult<(u8,)> = ic_cdk::call(
        Principal::from_text(TOKEN_CANISTER_ID).unwrap(),
        "icrc1_decimals",
        (),
    )
    .await;

    let fee: CallResult<(Balance,)> = ic_cdk::call(
        Principal::from_text(TOKEN_CANISTER_ID).unwrap(),
        "icrc1_fee",
        (),
    )
    .await;

    let min_burn_amount: CallResult<(Balance,)> = ic_cdk::call(
        Principal::from_text(TOKEN_CANISTER_ID).unwrap(),
        "min_burn_amount",
        (),
    )
    .await;

    match (name, symbol, total_supply, decimals, fee, min_burn_amount) {
        (
            Ok((name,)),
            Ok((symbol,)),
            Ok((total_supply,)),
            Ok((decimals,)),
            Ok((fee,)),
            Ok((min_burn_amount,)),
        ) => Ok((
            name,
            symbol,
            total_supply.0,
            decimals,
            fee.0,
            min_burn_amount.0,
        )),
        _ => Err("Failed to retrieve token info".to_string()),
    }
}

// ----------------------
// candid interface
// ----------------------
ic_cdk::export_candid!();
