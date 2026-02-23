interface CategoryEntry {
  total: number
  count: number
}

interface CategoryState {
  categories: Record<string, CategoryEntry>
}

interface ExpenseEvent {
  amount: number
  category: string
  description: string
}

fromCategory('account')
  .when({
    $init(): CategoryState {
      return { categories: {} }
    },
    expense(s: CategoryState, e: ProjectionEvent<ExpenseEvent>) {
      var cat = e.body.category
      if (!s.categories[cat]) {
        s.categories[cat] = { total: 0, count: 0 }
      }
      s.categories[cat].total += e.body.amount
      s.categories[cat].count += 1
    }
  })
  .outputState()
