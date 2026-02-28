SELECT balance, total_income, total_expense, event_count
FROM account_balances
WHERE account = $1
