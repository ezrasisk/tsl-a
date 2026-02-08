use reqwest::blocking::Client;
use serde_json::Value;
use chrono::Utc;

pub struct MarketData {
    pub price: f64,
    pub change_1h: f64,
    pub change_24h: f64,
    pub change_7d: f64,
    pub change_14d: f64,
    pub volume_24h: f64,
}

pub fn fetch_market_data(client: &Client) -> Result<MarketData, String> {
    let now = Utc::now();
    let timestamp_str = now.timestamp().to_string();

    let url = format!(
        "https://api.coingecko.com/api/v3/coins/bitcoin?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false&sparkline=false&_={}",
        timestamp_str
    );

    let request = client.get(&url)
        .header("Cache-Control", "no-cache, no-store, must-revalidate")
        .header("Pragma", "no-cache")
        .header("Expires", "0")
        .header("User-Agent", "Rust BTC Monitor/1.0");

    let resp = request.send().map_err(|e| format!("Network error: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let response: Value = resp.json().map_err(|e| format!("JSON parse error: {}", e))?;

    if response.get("error").is_some() {
        return Err("Rate limited or API error".to_string());
    }

    let market_data = response["market_data"].as_object()
        .ok_or("Missing market_data field")?;

    let price = market_data["current_price"]["usd"].as_f64().unwrap_or(0.0);
    let change_1h = market_data["price_change_percentage_1h_in_currency"]["usd"].as_f64().unwrap_or(0.0);
    let change_24h = market_data["price_change_percentage_24h_in_currency"]["usd"].as_f64().unwrap_or(0.0);
    let change_7d = market_data["price_change_percentage_7d_in_currency"]["usd"].as_f64().unwrap_or(0.0);
    let change_14d = market_data["price_change_percentage_14d_in_currency"]["usd"].as_f64().unwrap_or(0.0);
    let volume_24h = market_data["total_volume"]["usd"].as_f64().unwrap_or(1.0);

    Ok(MarketData {
        price,
        change_1h,
        change_24h,
        change_7d,
        change_14d,
        volume_24h,
    })
}
