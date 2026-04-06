use axum::{Router, routing::get};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{AppState, api::handlers::{feeds, health}};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/v1/feeds", get(feeds::list_feeds))
        .route("/api/v1/feeds/{feed_id}", get(feeds::get_feed))
        .route("/api/v1/feeds/{feed_id}/history", get(feeds::get_feed_history))
        .route("/api/v1/feeds/{feed_id}/update", get(feeds::force_update_feed))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}