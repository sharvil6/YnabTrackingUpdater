
use serde::{Deserialize, Serialize};

pub struct YnabMoney {
    pub milliunits: u64,
    pub dollars: u64,
    pub cents: u64,
    pub money_string: String,
    pub sign: i8
}

impl YnabMoney {
    // pub fn new(&mut self)
    // {
    //     self.milliunits = 0;
    //     self.dollars = 0;
    //     self.cents = 0;
    //     self.sign = 1;
    //     self.money_string = "0.00".to_string();
    // }

    pub fn new_from_milliunits(milliunits: i64) -> YnabMoney {
        let ynab_sign:i8;
        let ynab_dollars: u64;
        let ynab_cents: u64;
        let ynab_milliunits: u64;

        if milliunits < 0 {
            ynab_sign = -1;
            ynab_milliunits = milliunits.abs() as u64;
        }
        else {
            ynab_sign = 1;
            ynab_milliunits = milliunits.abs() as u64;
        }
        
        ynab_dollars = (ynab_milliunits / 1000) as u64;
        ynab_cents = ((ynab_milliunits % 1000) / 10) as u64;

        YnabMoney {
            milliunits: ynab_milliunits,
            dollars: ynab_dollars,
            cents: ynab_cents,
            money_string: format!("{}.{:02}", (ynab_sign as i64) * (ynab_dollars as i64), ynab_cents),
            sign: ynab_sign
        }
    }

    pub fn new_from_string(money_string: String) -> YnabMoney {
        let money_string = money_string.trim().to_string();
        let ynab_sign: i8;
        let ynab_dollars: u64;
        let ynab_cents: u64;
        let ynab_milliunits: u64;

        if money_string.chars().nth(0).unwrap() == '-' {
            ynab_sign = -1;
        }
        else {
            ynab_sign = 1;
        }

        let result = money_string.split_once('.');
        match result {
            Some(money_value) => {
                ynab_dollars = money_value.0.parse().unwrap();
                ynab_cents = money_value.1.parse().unwrap();
            }
            None => {
                println!{"ERROR! Failed to parse money string"};
                return YnabMoney {
                    milliunits: 0,
                    dollars: 0,
                    cents: 0,
                    sign: 0,
                    money_string: String::new()
                }
            }
        }

        ynab_milliunits = (ynab_dollars * 1000) + (ynab_cents * 10);

        YnabMoney {
            milliunits: ynab_milliunits,
            dollars: ynab_dollars,
            cents: ynab_cents, 
            money_string: money_string,
            sign: ynab_sign
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionInfo {
    transaction: TransactionData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionData {
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
pub struct Subtransactions {
    amount: i64,
    payee_id: String,
    payee_name: String,
    category_id: String,
    memo: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountsData {
    data: AccountResponse,
}

impl AccountsData {
    pub fn get_account_vec(&self) -> &Vec<Account> {
        &self.data.accounts
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountResponse {
    accounts: Vec<Account>,
    server_knowledge: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub name: String,
    r#type: String,
    on_budget: bool,
    closed: bool,
    note: Option<String>,
    pub balance: i64,
    pub cleared_balance: i64,
    pub uncleared_balance: i64,
    transfer_payee_id: String,
    deleted: bool,
}
