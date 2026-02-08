mod quaternion;
mod data;
mod predictor;

use quaternion::Quaternion;
use data::{fetch_market_data, MarketData};
use predictor::{compute_gauge_and_prediction, GaugeMetrics};
use reqwest::blocking::Client;
use std::thread;
use std::time::Duration;
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut prev_unit_q = Quaternion::new(1.0, 0.0, 0.0, 0.0);

    println!("BTC Quaternion Gauge + Lagrangian Predictor (CoinGecko)");
    println!("Live + Speculative Forecast every 45s | Ctrl+C to stop\n");

    loop {
        let timestamp_display = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();

        match fetch_market_data(&client) {
            Ok(data) => {
                let MarketData {
                    price,
                    change_1h,
                    change_24h,
                    change_7d,
                    change_14d,
                    volume_24h,
                } = data;

                let raw_q = Quaternion::new(change_24h, change_1h, change_7d, change_14d);

                let metrics: GaugeMetrics = compute_gauge_and_prediction(
                    raw_q,
                    volume_24h,
                    prev_unit_q,
                );

                prev_unit_q = raw_q.normalized();

                println!("[{}] Price: ${:.2}", timestamp_display, price);
                println!("Changes: 1h:{:.2}% 24h:{:.2}% 7d:{:.2}% 14d:{:.2}%",
                         change_1h, change_24h, change_7d, change_14d);
                println!("Current  — Norm: {:.2}  Angle: {:.1}°  Signal: {}",
                         metrics.norm, metrics.angle_deg, metrics.current_signal);
                println!("Lagrangian: {:.3} (higher = more favorable dynamics)", metrics.lagrangian);
                println!("Speculative → Predicted Angle: {:.1}°  Signal: {}\n",
                         metrics.predicted_angle, metrics.speculative_signal);
            }
            Err(e) => {
                println!("[{}] Error fetching data: {} (retrying...)\n", timestamp_display, e);
            }
        }

        thread::sleep(Duration::from_secs(45));
    }
}
