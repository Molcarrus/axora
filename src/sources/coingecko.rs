use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;

use crate::domain::data_point::FeedId;

pub struct CoinGeckoFetcher {
    client: Client,
    api_key: Option<String>,
}

#[derive(Deserialize)]
struct CoinGeckoResponse {
    #[serde(flatten)]
    prices: HashMap<String, CoinGeckoCoin>,
}

#[derive(Deserialize)]
struct CoinGeckoCoin {
    usd: f64,
    #[serde(default)]
    usd_24h_vol: Option<f64>,
    #[serde(default)]
    usd_market_cap: Option<f64>,
}

impl CoinGeckoFetcher {
    pub fn new(api_key: Option<String>) -> Self {
        Self { 
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("Failed to build HTTP client"), 
            api_key,
        }
    }
    
    fn feed_to_coint_id(feed_id: &FeedId) -> Option<&str> {
        match feed_id.0.as_str() {
            "ETH/USD" => Some("ethereum"),
            "BTC/USD" => Some("bitcoin"),
            "SOL/USD" => Some("solana"),
            "LINK/USD" => Some("chainlink"),
            "AVAX/USD" => Some("avalanche-2"),
            _ => None,
        }
    }
}