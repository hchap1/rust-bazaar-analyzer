use std::time::Instant;
use reqwest::{get, Response};
use reqwest::Error;

struct Product {
    buy_order_price: f32,
    current_buy_volume: usize,
    sell_order_price: f32,
    current_sell_volume: usize,
    product_id: String,
    moving_week: usize,
    margin: f32,
    seconds_between_sells: f32,
    weekly_moved_value: f32
}

struct BazaarData {
    timestamp: Instant,
    products: Vec<Product>
}

async fn get_bazaar_data() -> Result<BazaarData, String> {
    let url: &str = "https://api.hypixel.net/v2/skyblock/bazaar";
    let response: Result<Response, Error> = get(url).await;
    if let Ok(response) = response {
        if let Ok(raw_json) = response.text().await {
            if let Ok(data) = json::parse(&raw_json) {
                let success: bool = data["success"] == true;
                let mut bz_data: BazaarData = BazaarData { timestamp: Instant::now(), products: vec![] };
                if success {
                    for product in data["products"].entries() {
                        let product_data = product.1;
                        let product_id: String = product_data["product_id"].to_string();
                        let mut sell_order_price: f32 = -1f32;
                        let mut buy_order_price: f32 = -1f32;
                        if let Some(best_sell_order) = product_data["buy_summary"][0]["pricePerUnit"].as_f32() {
                            sell_order_price = best_sell_order;
                        }
                        if let Some(best_buy_order) = product_data["sell_summary"][0]["pricePerUnit"].as_f32() {
                            buy_order_price = best_buy_order;
                        }
                        let quick_status = &product_data["quick_status"];
                        let current_buy_volume = quick_status["buyVolume"].as_usize().unwrap();
                        let current_sell_volume = quick_status["sellVolume"].as_usize().unwrap();
                        let moving_week = quick_status["sellMovingWeek"].as_usize().unwrap() + quick_status["buyMovingWeek"].as_usize().unwrap();
                        let mut margin: f32 = sell_order_price - buy_order_price;
                        if sell_order_price < 0f32 || buy_order_price < 0f32 {
                            continue;
                        }
                        let seconds_between_sells = 1f32 / (moving_week as f32) * 604800f32;
                        let weekly_moved_value: f32 = moving_week as f32 * (sell_order_price + buy_order_price) / 2f32;
                        margin = margin / buy_order_price;
                        let mut index: Option<usize> = None;
                        for (i, p) in bz_data.products.iter().enumerate() {
                            if margin > p.margin {
                                index = Some(i);
                                break;
                            }
                        }
                        match index {
                            Some(idx) => {
                                bz_data.products.insert(idx, Product {
                                    buy_order_price,
                                    current_buy_volume,
                                    sell_order_price,
                                    current_sell_volume,
                                    product_id,
                                    moving_week,
                                    margin,
                                    seconds_between_sells,
                                    weekly_moved_value
                                })
                            }
                            None => { bz_data.products.push(Product { buy_order_price,
                                current_buy_volume,
                                sell_order_price,
                                current_sell_volume,
                                product_id,
                                moving_week,
                                margin,
                                seconds_between_sells,
                                weekly_moved_value
                                });
                            }
                        }
                    }
                    return Ok(bz_data);
                }
            }
        }
    }
    Err(String::from("API failure"))
}

#[tokio::main]
async fn main() {
    match get_bazaar_data().await {
        Ok(bz_data) => {
            for product in &bz_data.products {
                if product.margin > 0f32 {

                }
            }
        }
        Err(e) => { println!("API Error: {e}"); } 
    }
}
