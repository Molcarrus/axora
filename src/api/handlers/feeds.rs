use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::{Deserialize, Serialize};

use crate::{
    AppState, aggregation::engine::AggregationEngine, domain::data_point::VerifiedDataPoint, state,
};
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

#[derive(Deserialize)]
pub struct HistoryQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    100
}

pub async fn get_feed_history(
    State(state): State<AppState>,
    Path(feed_id): Path<String>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<Vec<VerifiedDataPoint>>, OracleError> {
    let feed_id = FeedId::new(feed_id);
    let limit = query.limit.min(1000);

    let history = state.storage.get_history(&feed_id, limit).await?;

    Ok(Json(history))
}

pub async fn force_update_feed(
    State(state): State<AppState>,
    Path(feed_id): Path<String>,
) -> Result<Json<VerifiedDataPoint>, OracleError> {
    let feed_id = FeedId::new(feed_id);

    let _ = state.feed_manager.get_feed(&feed_id)?;

    if !state.data_fetcher.supports_feed(&feed_id) {
        return Err(OracleError::BadRequest(format!(
            "Feed {} not supported by data source",
            feed_id.0
        )));
    }

    let raw_data = state.data_fetcher.fetch(&feed_id).await?;

    tracing::info!(
        feed_id = %feed_id.0, 
        source = raw_data.source, 
        "Fetched data successfully"
    );

    let verified = AggregationEngine::aggregate_simple(raw_data)?;

    state.storage.store_data_point(&verified).await?;

    state.feed_manager.update_feed(&verified)?;

    tracing::info!(
        feed_id = %feed_id.0, 
        "Feed updated successfully"
    );

    Ok(Json(verified))
}
