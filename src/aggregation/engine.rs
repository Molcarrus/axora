use chrono::Utc;
use uuid::Uuid;

use crate::{
    domain::data_point::{RawDataPoint, VerifiedDataPoint},
    error::{OracleError, Result},
};

pub struct AggregationEngine;

impl AggregationEngine {
    pub fn aggregate_simple(raw_point: RawDataPoint) -> Result<VerifiedDataPoint> {
        if raw_point.confidence <= 0.0 {
            return Err(OracleError::BadRequest(
                "Invalid confidence value".to_string(),
            ));
        }

        Ok(VerifiedDataPoint {
            id: Uuid::new_v4(),
            feed_id: raw_point.feed_id,
            value: raw_point.value,
            timestamp: Utc::now(),
            confidence: raw_point.confidence,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::data_point::{DataValue, FeedId, PriceData};

    use super::*;

    #[test]
    fn test_aggregate_simple() {
        let raw = RawDataPoint {
            id: Uuid::new_v4(),
            feed_id: FeedId::new("ETH/USD"),
            source: "test".to_string(),
            value: DataValue::Price(PriceData {
                price: 2500.0,
                currency: "USD".to_string(),
                volume_24h: None,
                market_cap: None,
            }),
            timestamp: Utc::now(),
            confidence: 0.9,
        };

        let result = AggregationEngine::aggregate_simple(raw);
        assert!(result.is_ok());

        let verified = result.unwrap();
        assert_eq!(verified.feed_id.0, "ETH/USD");
        assert_eq!(verified.confidence, 0.9);
    }
}
