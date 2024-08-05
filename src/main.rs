use std::time::Instant;

use reqwest::blocking::{get, Response};
use reqwest::Error;

struct Product {
    buy_order_price: f32,
    current_buy_volume: usize,
    sell_order_price: f32,
    current_sell_volume: usize,
    product_id: String,
    moving_week: usize
}

struct BazaarData {
    timestamp: Instant,
    products: Vec<Product>
}

fn main() {
    let url: &str = "https://api.hypixel.net/v2/skyblock/bazaar";
    let response: Result<Response, Error> = get(url);
    if let Ok(response) = response {
        if let Ok(raw_json) = response.text() {
            if let Ok(data) = json::parse(&raw_json) {
                let success: bool = data["success"] == true;
                if success {
                    
                }
            }
        }
    }
}
