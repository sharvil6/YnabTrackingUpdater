use std::{collections::HashMap, fs, io};

mod ynab_json_structures;

const LAST_USED_BUDGET_BASE_URL: &str = "https://api.youneedabudget.com/v1/budgets/last-used/accounts?access_token=";

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


}
