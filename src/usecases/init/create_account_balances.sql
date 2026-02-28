CREATE TABLE IF NOT EXISTS account_balances (
    account TEXT PRIMARY KEY,
    balance BIGINT NOT NULL DEFAULT 0,
    total_income BIGINT NOT NULL DEFAULT 0,
    total_expense BIGINT NOT NULL DEFAULT 0,
    event_count BIGINT NOT NULL DEFAULT 0
)
