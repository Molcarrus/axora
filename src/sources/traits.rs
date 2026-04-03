use async_trait::async_trait;

use crate::{
    domain::data_point::{FeedId, RawDataPoint},
    error::Result,
};

#[async_trait]
pub trait DataFetcher: Send + Sync + 'static {
    async fn fetch(&self, feed_id: &FeedId) -> Result<RawDataPoint>;

    fn supports_feed(&self, feed_id: &FeedId) -> bool;

    fn source_name(&self) -> &str;
}
