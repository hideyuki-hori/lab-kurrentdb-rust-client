INSERT INTO account_balances (account, balance, total_expense, event_count)
VALUES ($1, -$2, $2, 1)
ON CONFLICT (account) DO UPDATE SET
    balance = account_balances.balance - $2,
    total_expense = account_balances.total_expense + $2,
    event_count = account_balances.event_count + 1
