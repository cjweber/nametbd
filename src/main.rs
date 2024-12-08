use reqwest::Error;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct PriceResponse {
    #[serde(flatten)]
    prices: std::collections::HashMap<String, std::collections::HashMap<String, f64>>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: crypto_price <coin1> <coin2> ...");
        std::process::exit(1);
    }

    let coins = args[1..].join(",");
    let currency = "usd";

    let api_url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        coins, currency
    );

    let response = reqwest::get(&api_url).await?;

    let price_response: PriceResponse = response.json().await?;

    let first_coin = &args[1];
    let first_coin_price = match price_response.prices.get(first_coin) {
        Some(data) => data.get(currency),
        None => None,
    };

    if first_coin_price.is_none() {
        eprintln!("Could not retrieve price for '{}'.", first_coin);
        std::process::exit(1);
    }

    let first_coin_price = first_coin_price.unwrap();

    println!(
        "The current price of {} is {:.2} USD.",
        first_coin, first_coin_price
    );

    for coin_id in args[2..].iter() {
        if let Some(coin_data) = price_response.prices.get(coin_id) {
            if let Some(price) = coin_data.get(currency) {
                println!(
                    "The current price of {} is {:.2} USD.",
                    coin_id, price
                );
                let rate = first_coin_price / price;
                println!("1 {} equals {:.2} {}.", first_coin, rate, coin_id);
            } else {
                eprintln!("USD price not found for coin '{}'.", coin_id);
            }
        } else {
            eprintln!("Coin '{}' not found.", coin_id);
        }
    }

    Ok(())
}

