use std::{collections::HashMap, fs, io};
use reqwest::blocking::Client;
use serde_json::json;
use chrono::{Month, Datelike, TimeZone, Utc};
use num_traits::cast::FromPrimitive;

use crate::ynab_json_structures::YnabMoney;

mod ynab_json_structures;

struct ModifiedAccounts {
    account_id: String,
    name: String,
    current_balance: ynab_json_structures::YnabMoney,
    adjustment: ynab_json_structures::YnabMoney
}

const LAST_USED_BUDGET_GET_ACCOUNTS_BASE_URL: &str = "https://api.youneedabudget.com/v1/budgets/last-used/accounts?access_token=";
const LAST_USED_BUDGET_POST_TRANSACTION_BASE_URL: &str = "https://api.youneedabudget.com/v1/budgets/last-used/transactions?access_token=";

fn parse_api_token_file(filepath: &str, token_dict: &mut HashMap<String, String>)
{
    let api_tokens  = fs::read_to_string(filepath).expect("Error, could not open file");
    let lines = api_tokens.lines();
    for line in lines {
        let result = line.split_once('=');
        match result {
            Some(key_value) => {
                token_dict.insert(key_value.0.to_string().to_uppercase(), key_value.1.to_string());
            },
            None => {
                println!("Error reading api tokens");
            },
        }
    }

}

// take input month, return string of date
fn get_adjustment_date() -> String {

    String::new()
}

fn main() {
    let mut api_token_dictionary:HashMap<String, String> = HashMap::new();
    parse_api_token_file("src/api_tokens.env", &mut api_token_dictionary);
    
    let mut access_token = None;
    while access_token == None {
        println!("\nSelect a user by typing their name:");
        for key in api_token_dictionary.keys() {
            println!("--{}", key);
        }
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read user input");
        access_token = api_token_dictionary.get(&user_input.trim().to_uppercase());
    }

    let url = format!("{}{}", LAST_USED_BUDGET_GET_ACCOUNTS_BASE_URL, access_token.unwrap());
    let blocking_client = Client::new();
    let accounts_resp = blocking_client.get(url.as_str()).send();

    if accounts_resp.is_ok() {
        let accounts_text = accounts_resp.unwrap().text().unwrap();
        let accounts: Result< ynab_json_structures::AccountsData, _> = serde_json::from_str(accounts_text.as_str());
        let mut modified_accounts: Vec<ModifiedAccounts> = Vec::new();

        if accounts.is_ok() {
            for (i, account) in accounts.as_ref().unwrap().get_account_vec().iter().enumerate() {
                println!{"[{}] -- {}", i, account.name}
                
                let adjustment = ModifiedAccounts {
                    account_id: account.id.to_string(),
                    name: account.name.to_string(),
                    current_balance: YnabMoney::new_from_milliunits(account.balance),
                    adjustment: YnabMoney::new_from_milliunits(0)
                };
                modified_accounts.push(adjustment);
            }
            
            let mut user_input:String = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read user input");

            while user_input.trim().to_uppercase() != "Q" {

                let account_index: usize = user_input.trim().parse().expect("Please type a number");
                let account_info = accounts.as_ref().unwrap().get_account_vec().get(account_index);
                
                match account_info {
                    Some(account) => {
                        println!("You selected: {}", account.name);
                        let current_balance = ynab_json_structures::YnabMoney::new_from_milliunits(account.balance);
                        let mut new_balance = String::new();
                        println!("New Balance: $");
                        io::stdin().read_line(&mut new_balance).expect("Failed to read user input");
                        
                        let new_balance = ynab_json_structures::YnabMoney::new_from_string(new_balance);
                        let adjustment_amt = ynab_json_structures::YnabMoney::new_from_milliunits( ( (new_balance.milliunits as i64) - (current_balance.milliunits as i64)) as i64);
                        println!("Balance Adjustment: ${}", adjustment_amt.money_string);
                        
                        modified_accounts.get_mut(account_index).unwrap().adjustment = adjustment_amt;

                        for (i, acc) in modified_accounts.iter().enumerate() {
                            if acc.adjustment.milliunits != 0 {
                                println!("[{}] -- {}: Adjustment --> ${}", i, acc.name, acc.adjustment.money_string);
                            }
                            else {
                                println!("[{}] -- {}:", i, acc.name);
                            }
                        }
                        
                    }
                    None => {
                        continue;
                    }
                }

                user_input.clear();
                io::stdin().read_line(&mut user_input).expect("Failed to read user input");
                
            }

            println!("\n\nEnter month of adjustment: ");
            user_input.clear();
            io::stdin().read_line(&mut user_input).expect("Failed to read user input");
            
            let month_number: Month = Month::from_u32(user_input.trim().parse().unwrap()).unwrap();
            let mut next_month = Utc.ymd(Utc::now().date().year(), month_number.succ().number_from_month(), 1);
            if month_number == Month::December {
                next_month = next_month.with_year(next_month.year() + 1).unwrap();
            }
            let transaction_date = next_month.pred();

            for modification in modified_accounts.iter() {
                if modification.adjustment.milliunits != 0 {
                    let transaction_data = json!({
                        "transactions":[
                            {
                                "account_id": modification.account_id,
                                "date": transaction_date.format("%Y-%m-%d").to_string(),
                                "amount": (modification.adjustment.sign as i64) * (modification.adjustment.milliunits as i64),
                                "memo": "Market Change & Dividends",
                                "cleared": "cleared",
                                "approved": true
                            }
                        ]
                    });

                    println!("Submitting adjustment for {} ...", modification.name);
                    println!("JSON  : {:?}", transaction_data);
                    
                    let url = format!("{}{}", LAST_USED_BUDGET_POST_TRANSACTION_BASE_URL, access_token.unwrap());
                    let result = blocking_client.post(url.as_str())
                                                .json(&transaction_data)
                                                .send();
                    println!("{:?}", result);
                    match result {
                        Ok(response) => {
                            if response.status().as_u16() == 201 {
                                println!("Adjustment added successfully!");
                            }
                            else {
                                println!("Error :( Status Code = {:?}", response.status());
                            }
                        }
                        Err(err) => {
                            println!("Post.Send Error: {:?}", err);
                        }
                    }
                }
            }
            
            
        }
    }
    
}
