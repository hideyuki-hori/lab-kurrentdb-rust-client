interface BalanceState {
  balance: number
  total_income: number
  total_expense: number
  event_count: number
}

interface IncomeEvent {
  amount: number
  description: string
}

interface ExpenseEvent {
  amount: number
  category: string
  description: string
}

fromCategory('account')
  .foreachStream()
  .when({
    $init(): BalanceState {
      return { balance: 0, total_income: 0, total_expense: 0, event_count: 0 }
    },
    income(s: BalanceState, e: ProjectionEvent<IncomeEvent>) {
      s.balance += e.body.amount
      s.total_income += e.body.amount
      s.event_count += 1
    },
    expense(s: BalanceState, e: ProjectionEvent<ExpenseEvent>) {
      s.balance -= e.body.amount
      s.total_expense += e.body.amount
      s.event_count += 1
    }
  })
  .outputState()
