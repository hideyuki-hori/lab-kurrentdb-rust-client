interface AlertTotals {
  totals: Record<string, number>
}

interface ExpenseEvent {
  amount: number
  category: string
  description: string
}

fromCategory('account')
  .when({
    $init(): AlertTotals {
      return { totals: {} }
    },
    expense(s: AlertTotals, e: ProjectionEvent<ExpenseEvent>) {
      var cat = e.body.category
      if (!s.totals[cat]) s.totals[cat] = 0
      s.totals[cat] += e.body.amount

      if (s.totals[cat] > 50000) {
        var account = e.streamId.replace('account-', '')
        emit('alert-' + account, 'BudgetExceeded', {
          category: cat,
          total: s.totals[cat],
          triggered_by: e.body.amount,
          account: account
        })
      }
    }
  })
