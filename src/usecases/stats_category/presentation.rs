use crate::domain::Amount;
use crate::domain::query::CategoryExpense;

pub struct View;

impl super::Presenter for View {
    fn render(&self, rows: &[CategoryExpense]) {
        if rows.is_empty() {
            println!("No expense data found.");
            return;
        }

        println!("{:<15} {:>12} {:>6}", "CATEGORY", "TOTAL", "COUNT");
        println!("{}", "-".repeat(35));

        for r in rows {
            println!(
                "{:<15} {:>12} {:>6}",
                r.category,
                Amount::from_u64(r.total.unsigned_abs()),
                r.count,
            );
        }
    }
}
