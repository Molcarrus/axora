use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    config::AppConfig,
    sources::{coingecko::CoinGeckoFetcher, traits::DataFetcher},
    state::feed_manager::FeedManager,
    storage::postgres::PostrgresStorage,
};

pub mod aggregation;
pub mod api;
pub mod config;
pub mod domain;
pub mod error;
pub mod sources;
pub mod state;
pub mod storage;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: PgPool,
    pub storage: Arc<PostrgresStorage>,
    pub feed_manager: Arc<FeedManager>,
    pub data_fetcher: Arc<dyn DataFetcher>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> anyhow::Result<Self> {
        let db = PgPool::connect(&config.database.url).await?;

        sqlx::migrate!("./migrations").run(&db).await?;
        tracing::info!("Database migrations completed");

        let storage = Arc::new(PostrgresStorage::new(db.clone()));

        let feed_manager = Arc::new(FeedManager::new());
        feed_manager.initialize_default_feeds();

        let data_fetcher: Arc<dyn DataFetcher> = Arc::new(CoinGeckoFetcher::new(
            config.sources.coingecko_api_key.clone(),
        ));

        Ok(Self {
            config: Arc::new(config),
            db,
            storage,
            feed_manager,
            data_fetcher,
        })
    }
}
