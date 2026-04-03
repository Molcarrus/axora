use chrono::Utc;
use uuid::Uuid;

use crate::{domain::data_point::{RawDataPoint, VerifiedDataPoint}, error::{OracleError, Result}};

pub struct AggregationEngine;

impl AggregationEngine {
    pub fn aggregate_simple(raw_point: RawDataPoint) -> Result<VerifiedDataPoint> {
        if raw_point.confidence <= 0.0 {
            return Err(OracleError::BadRequest(
                "Invalid confidence value".to_string(),
            ));
        }
        
        Ok(VerifiedDataPoint { id: Uuid::new_v4(), feed_id: raw_point.feed_id, value: raw_point.value, timestamp: Utc::now(), confidence: raw_point.confidence })
    }
}