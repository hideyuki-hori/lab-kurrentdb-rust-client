INSERT INTO account_balances (account, balance, total_income, event_count)
VALUES ($1, $2, $2, 1)
ON CONFLICT (account) DO UPDATE SET
    balance = account_balances.balance + $2,
    total_income = account_balances.total_income + $2,
    event_count = account_balances.event_count + 1
