use reqwest;
use serde::Deserialize;
use clap::Parser;
use serde_json::Value;
use std::error::Error;
use std::io::{self, Write}; 
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not found in .env file");

    println!("{}", api_key);

    println!("-----------------------------------");
    println!("       Currency Converter");
    println!("-----------------------------------\n");

    // Function to get user input
    fn get_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    }

    let currency_from = get_input("Enter currency from (e.g., USD): ").to_uppercase();
    let amount: f64 = loop {
        let amount_str = get_input("Enter amount: ");
        match amount_str.parse::<f64>() {
            Ok(num) => break num,
            Err(_) => println!("Invalid amount. Please enter a valid number."),
        }
    };
    let currency_to = get_input("Enter currency to (e.g., EUR): ").to_uppercase();

    println!("\nConverting {:.2} {} to {}", amount, currency_from, currency_to);
    println!("------------------------");
    println!("------------------------");
 
    match get_conversion_rates(&api_key, &currency_from, &currency_to).await {
        Ok(rate) => println!("{} {} is equivalent to {:.2} {}", amount, currency_from, rate * amount, currency_to ),
        Err(_) => eprintln!("Error"),
    }

    

    // API request will be added here in the future
}

pub async fn get_conversion_rates(api_key: &str, from: &str, to: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!("https://v6.exchangerate-api.com/v6/{}/latest/{}", api_key, from);

    let response = reqwest::get(&url).await?.json::<Value>().await?;git 

    println!("{}", response);

    if let Some(rates) = response["conversion_rates"].as_object() {
        if let Some(rate) = rates.get(to).and_then(|v| v.as_f64()) {
            return Ok(rate);
        } else {
            return Err(format!("Currency code '{}' not found in conversion rates.", to).into());
        }
    } else {
        return Err("Invalid API response: Missing 'conversion_rates' field.".into());
    }
    
}