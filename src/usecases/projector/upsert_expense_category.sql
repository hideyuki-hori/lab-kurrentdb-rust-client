INSERT INTO category_expenses (category, total, count)
VALUES ($1, $2, 1)
ON CONFLICT (category) DO UPDATE SET
    total = category_expenses.total + $2,
    count = category_expenses.count + 1
