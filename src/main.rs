use reqwest;
use serde::Deserialize;
use clap::Parser;
use std::error::Error;
use std::io::{self, Write}; 
use std::env;
use dotenv::dotenv;

fn main() {
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

    // API request will be added here in the future
}
