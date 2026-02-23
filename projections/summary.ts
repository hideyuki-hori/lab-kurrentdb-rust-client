interface SummaryState {
  total_income: number
  total_expense: number
  net: number
  accounts: Record<string, number>
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
  .when({
    $init(): SummaryState {
      return { total_income: 0, total_expense: 0, net: 0, accounts: {} }
    },
    income(s: SummaryState, e: ProjectionEvent<IncomeEvent>) {
      if (!s.accounts[e.streamId]) s.accounts[e.streamId] = 0
      s.accounts[e.streamId] += e.body.amount
      s.total_income += e.body.amount
      s.net += e.body.amount
    },
    expense(s: SummaryState, e: ProjectionEvent<ExpenseEvent>) {
      if (!s.accounts[e.streamId]) s.accounts[e.streamId] = 0
      s.accounts[e.streamId] -= e.body.amount
      s.total_expense += e.body.amount
      s.net -= e.body.amount
    }
  })
  .outputState()
