INSERT INTO checkpoints (id, commit_position, prepare_position)
VALUES ($1, $2, $3)
ON CONFLICT (id) DO UPDATE SET
    commit_position = $2,
    prepare_position = $3
