use reqwest;
use serde::Deserialize;
use clap::Parser;
use std::error::Error;
use std::io::{self, Write}; 
use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not found in .env file");

    println!("{}", api_key);

    println!("-----------------------------------");
    println!("       Currency Converter");
    println!("-----------------------------------");
    println!(" ");

    print!("Enter currency from: ");
    io::stdout().flush().unwrap();
    let mut currency_from = String::new();
    io::stdin().read_line(&mut currency_from).expect("Failed to read line");
    let currency_from = currency_from.trim();

    print!("Enter amount: ");
    io::stdout().flush().unwrap();
    let mut amount_input = String::new();
    io::stdin().read_line(&mut amount_input).expect("Failed to read line");
    let amount: f64 = amount_input.trim().parse().expect("Please enter a valid number");

    print!("Enter currency to: ");
    io::stdout().flush().unwrap();
    let mut currency_to  = String::new();
    io::stdin().read_line(&mut currency_to).expect("Failed to read line");
    let currency_to = currency_to.trim();

    println!("\nConverting {} {} to {}",currency_from, amount, currency_to);

    
}

