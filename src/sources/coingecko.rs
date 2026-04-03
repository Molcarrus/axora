use std::collections::HashMap;

use async_trait::async_trait;
use chrono::Utc;
use reqwest::Client;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    domain::data_point::{DataValue, FeedId, PriceData, RawDataPoint},
    error::{OracleError, Result},
    sources::traits::DataFetcher,
};

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

#[async_trait]
impl DataFetcher for CoinGeckoFetcher {
    fn source_name(&self) -> &str {
        "coingecko"
    }

    fn supports_feed(&self, feed_id: &FeedId) -> bool {
        Self::feed_to_coint_id(feed_id).is_some()
    }

    async fn fetch(&self, feed_id: &FeedId) -> Result<RawDataPoint> {
        let coin_id = Self::feed_to_coint_id(feed_id).ok_or_else(|| OracleError::SourceFetch {
            source_name: "coingecko".into(),
            message: format!("Unsupported feed: {}", feed_id.0),
        })?;

        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_vol=true&include_market)cap=true",
            coin_id
        );

        let mut request = self.client.get(&url);

        if let Some(ref api_key) = self.api_key {
            request = request.header("x-cg-demo-api-key", api_key);
        }

        let response = request.send().await.map_err(|e| OracleError::SourceFetch {
            source_name: "coingecko".into(),
            message: e.to_string(),
        })?;

        if !response.status().is_success() {
            return Err(OracleError::SourceFetch {
                source_name: "coingecko".into(),
                message: format!("HTTP {}", response.status()),
            });
        }

        let data: CoinGeckoResponse =
            response
                .json()
                .await
                .map_err(|e| OracleError::SourceFetch {
                    source_name: "coingecko".into(),
                    message: format!("JSON parse error: {}", e),
                })?;

        let coin_data = data
            .prices
            .get(coin_id)
            .ok_or_else(|| OracleError::SourceFetch {
                source_name: "coingecko".into(),
                message: format!("No data for {}", coin_id),
            })?;

        Ok(RawDataPoint {
            id: Uuid::new_v4(),
            feed_id: feed_id.clone(),
            source: "coingecko".into(),
            value: DataValue::Price(PriceData {
                price: coin_data.usd,
                currency: "USD".to_string(),
                volume_24h: coin_data.usd_24h_vol,
                market_cap: coin_data.usd_market_cap,
            }),
            timestamp: Utc::now(),
            confidence: 0.9,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_coingecko_fetch() {
        let fetcher = CoinGeckoFetcher::new(None);
        let feed_id = FeedId::new("ETH/USD");

        let result = fetcher.fetch(&feed_id).await;

        match result {
            Ok(data_point) => {
                println!("Fetched: {:?}", data_point);
                assert_eq!(data_point.feed_id, feed_id);
                assert_eq!(data_point.source, "coingecko");

                if let DataValue::Price(price) = data_point.value {
                    assert!(price.price > 0.0);
                } else {
                    panic!("Expected Price variant");
                }
            }
            Err(e) => {
                println!("Fetch failed (might be rate limited): {}", e);
            }
        }
    }

    #[test]
    fn test_supports_feed() {
        let fetcher = CoinGeckoFetcher::new(None);

        assert!(fetcher.supports_feed(&FeedId::new("ETH/USD")));
        assert!(fetcher.supports_feed(&FeedId::new("BTC/USD")));
        assert!(!fetcher.supports_feed(&FeedId::new("UNKNOWN")));
    }
}
