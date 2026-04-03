use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct FeedId(pub String);

impl FeedId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawDataPoint {
    pub id: Uuid,
    pub feed_id: FeedId,
    pub source: String,
    pub value: DataValue,
    pub timestamp: DateTime<Utc>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedDataPoint {
    pub id: Uuid,
    pub feed_id: FeedId,
    pub value: DataValue,
    pub timestamp: DateTime<Utc>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum DataValue {
    Price(PriceData),
    Generic(serde_json::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub price: f64,
    pub currency: String,
    pub volume_24h: Option<f64>,
    pub market_cap: Option<f64>,
}
