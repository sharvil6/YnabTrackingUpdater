
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TransactionInfo {
    transaction: TransactionData
}

#[derive(Serialize, Deserialize, Debug)]
struct TransactionData {
    account_id: String,
    date: String,
    amount: i64,
    payee_id: String,
    payee_name: String,
    category_id: String,
    memo: String,
    cleared: bool,
    approved: bool,
    flag_color: String,
    import_id: String,
    subtransactions: Vec<Subtransactions>
}

#[derive(Serialize, Deserialize, Debug)]
struct Subtransactions {
    amount: i64,
    payee_id: String,
    payee_name: String,
    category_id: String,
    memo: String
}

#[derive(Serialize, Deserialize, Debug)]
struct YNAB {
    data: AccountResponse,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountResponse {
    accounts: Vec<Account>,
    server_knowledge: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    id: String,
    name: String,
    r#type: String,
    on_budget: bool,
    closed: bool,
    note: Option<String>,
    balance: i64,
    cleared_balance: i64,
    uncleared_balance: i64,
    transfer_payee_id: String,
    deleted: bool,
}
