use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::data_point::{FeedId, VerifiedDataPoint};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedConfig {
    pub id: FeedId,
    pub name: String,
    pub category: FeedCategory,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedCategory {
    Crypto,
    Forex,
    Commodities,
    Weather,
}

#[derive(Debug, Clone)]
pub struct FeedState {
    pub config: FeedConfig,
    pub last_value: Option<VerifiedDataPoint>,
    pub last_update: Option<DateTime<Utc>>,
    pub error_count: u32,
}

impl FeedState {
    pub fn new(config: FeedConfig) -> Self {
        Self {
            config,
            last_value: None,
            last_update: None,
            error_count: 0,
        }
    }
}
