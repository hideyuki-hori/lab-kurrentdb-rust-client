use crate::domain::Amount;
use super::operation;

fn format_signed(n: i64) -> String {
    let abs = Amount::from_u64(n.unsigned_abs());
    if n < 0 {
        format!("-{}", abs)
    } else {
        format!("{}", abs)
    }
}

pub struct View;

impl super::Presenter for View {
    fn render(&self, output: &operation::Output) {
        if output.accounts.is_empty() {
            println!("No data found.");
            return;
        }

        println!("=== Account Summary ===\n");
        println!("  income:  {:>12}", Amount::from_u64(output.total_income.unsigned_abs()));
        println!("  expense: {:>12}", Amount::from_u64(output.total_expense.unsigned_abs()));
        println!("  net:     {:>12}", format_signed(output.net));

        println!("\n{:<25} {:>12}", "ACCOUNT", "BALANCE");
        println!("{}", "-".repeat(39));

        for r in &output.accounts {
            println!("{:<25} {:>12}", r.account, format_signed(r.balance));
        }
    }
}
