use std::sync::Arc;

use dashmap::DashMap;

use crate::{
    domain::{
        data_point::{FeedId, VerifiedDataPoint},
        feed::{FeedCategory, FeedConfig, FeedState},
    },
    error::{OracleError, Result},
};

pub struct FeedManager {
    feeds: Arc<DashMap<FeedId, FeedState>>,
}

impl FeedManager {
    pub fn new() -> Self {
        Self {
            feeds: Arc::new(DashMap::new()),
        }
    }

    pub fn initialize_default_feeds(&self) {
        let default_feeds = vec![
            FeedConfig {
                id: FeedId::new("ETH/USD"),
                name: "Ethereum / US Dollar".into(),
                category: FeedCategory::Crypto,
                active: true,
            },
            FeedConfig {
                id: FeedId::new("BTC/USD"),
                name: "Bitcoin / US Dollar".into(),
                category: FeedCategory::Crypto,
                active: true,
            },
            FeedConfig {
                id: FeedId::new("SOL/USD"),
                name: "Solana / US Dollar".into(),
                category: FeedCategory::Crypto,
                active: true,
            },
        ];

        for config in default_feeds {
            let feed_id = config.id.clone();
            self.feeds.insert(feed_id, FeedState::new(config));
        }

        tracing::info!("Initialized {} default feeds", self.feeds.len());
    }

    pub fn get_all_feeds(&self) -> Vec<FeedState> {
        self.feeds
            .iter()
            .filter(|entry| entry.value().config.active)
            .map(|entry| entry.value().clone())
            .collect()
    }

    pub fn get_feed(&self, feed_id: &FeedId) -> Result<FeedState> {
        self.feeds
            .get(feed_id)
            .map(|entry| entry.value().clone())
            .ok_or_else(|| OracleError::FeedNotFound(feed_id.0.clone()))
    }

    pub fn update_feed(&self, verified: &VerifiedDataPoint) -> Result<()> {
        let mut entry = self
            .feeds
            .get_mut(&verified.feed_id)
            .ok_or_else(|| OracleError::FeedNotFound(verified.feed_id.0.clone()))?;

        let state = entry.value_mut();
        state.last_value = Some(verified.clone());
        state.last_update = Some(verified.timestamp);
        state.error_count = 0;

        Ok(())
    }

    pub fn record_error(&self, feed_id: &FeedId) {
        if let Some(mut entry) = self.feeds.get_mut(feed_id) {
            entry.error_count += 1;
        }
    }
}
