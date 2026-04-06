use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;

use crate::AppState;
use crate::{domain::data_point::FeedId, error::OracleError};

#[derive(Serialize)]
pub struct FeedResponse {
    pub id: String,
    pub name: String,
    pub category: String,
    pub active: bool,
    pub latest: Option<LatestData>,
}

#[derive(Serialize)]
pub struct LatestData {
    pub value: serde_json::Value,
    pub timestamp: String,
    pub confidence: f64,
}

pub async fn list_feeds(
    State(state): State<AppState>,
) -> Result<Json<Vec<FeedResponse>>, OracleError> {
    let feeds = state.feed_manager.get_all_feeds();

    let response = feeds
        .into_iter()
        .map(|feed| FeedResponse {
            id: feed.config.id.0.clone(),
            name: feed.config.name.clone(),
            category: format!("{:?}", feed.config.category),
            active: feed.config.active,
            latest: feed.last_value.map(|v| LatestData {
                value: serde_json::to_value(&v.value).unwrap_or_default(),
                timestamp: v.timestamp.to_rfc3339(),
                confidence: v.confidence,
            }),
        })
        .collect();

    Ok(Json(response))
}

pub async fn get_feed(
    State(state): State<AppState>,
    Path(feed_id): Path<String>,
) -> Result<Json<FeedResponse>, OracleError> {
    let feed_id = FeedId::new(feed_id);
    let feed = state.feed_manager.get_feed(&feed_id)?;

    let response = FeedResponse {
        id: feed.config.id.0.clone(),
        name: feed.config.name.clone(),
        category: format!("{:?}", feed.config.category),
        active: feed.config.active,
        latest: feed.last_value.map(|v| LatestData {
            value: serde_json::to_value(&v.value).unwrap_or_default(),
            timestamp: v.timestamp.to_rfc3339(),
            confidence: v.confidence,
        }),
    };

    Ok(Json(response))
}
