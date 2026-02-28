SELECT commit_position, prepare_position
FROM checkpoints
WHERE id = $1
