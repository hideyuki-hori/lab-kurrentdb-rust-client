CREATE TABLE IF NOT EXISTS checkpoints (
    id TEXT PRIMARY KEY,
    commit_position BIGINT NOT NULL,
    prepare_position BIGINT NOT NULL
)
