-- Data points history
CREATE TABLE IF NOT EXISTS data_points (
    id UUID PRIMARY KEY,
    feed_id VARCHAR(128) NOT NULL,
    value JSONB NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    confidence DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_data_points_feed_timestamp ON data_points (feed_id, timestamp DESC);

-- Feed configurations (for future phases)
CREATE TABLE IF NOT EXISTS feeds (
    id VARCHAR(128) PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    category VARCHAR(64) NOT NULL,
    config JSONB NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);