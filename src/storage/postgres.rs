use sqlx::PgPool;

use crate::{
    domain::data_point::{FeedId, VerifiedDataPoint},
    error::Result,
};

pub struct PostrgresStorage {
    pool: PgPool,
}

impl PostrgresStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn store_data_point(&self, data: &VerifiedDataPoint) -> Result<()> {
        sqlx::query(
            r#"
            insert into data_points(id, feed_id, value, timestamp, confidence),
            values ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(data.id)
        .bind(&data.feed_id.0)
        .bind(serde_json::to_value(&data.value)?)
        .bind(data.timestamp)
        .bind(data.confidence)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_latest(&self, feed_id: &FeedId) -> Result<Option<VerifiedDataPoint>> {
        let row = sqlx::query_as::<
            _,
            (
                uuid::Uuid,
                String,
                serde_json::Value,
                chrono::DateTime<chrono::Utc>,
                f64,
            ),
        >(
            r#"
            select id, feed_id, value, timestamp, confidence
            from data_points
            where feed_id = $1
            order by timestamp desc
            limit 1
            "#,
        )
        .bind(&feed_id.0)
        .fetch_optional(&self.pool)
        .await?;

        if let Some((id, feed_id_str, value, timestamp, confidence)) = row {
            Ok(Some(VerifiedDataPoint {
                id,
                feed_id: FeedId::new(feed_id_str),
                value: serde_json::from_value(value)?,
                timestamp,
                confidence,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_history(
        &self,
        feed_id: &FeedId,
        limit: i64,
    ) -> Result<Vec<VerifiedDataPoint>> {
        let rows = sqlx::query_as::<
            _,
            (
                uuid::Uuid,
                String,
                serde_json::Value,
                chrono::DateTime<chrono::Utc>,
                f64,
            ),
        >(
            r#"
            select id, feed_id, value, timestamp, confidence
            from data_points
            where feed_id = $1
            order by timestamp desc
            limit $2
            "#,
        )
        .bind(&feed_id.0)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let data_points = rows
            .into_iter()
            .map(|(id, feed_id_str, value, timestamp, confidence)| {
                Ok(VerifiedDataPoint {
                    id,
                    feed_id: FeedId::new(feed_id_str),
                    value: serde_json::from_value(value)?,
                    timestamp,
                    confidence,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(data_points)
    }
}
