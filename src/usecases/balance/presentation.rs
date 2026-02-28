use crate::domain::Account;
use crate::domain::query::AccountBalance;
use crate::domain::Amount;

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
    fn render(&self, account: &Account, result: Option<AccountBalance>) {
        match result {
            Some(b) => {
                println!("{}: {} ({} events)", account, format_signed(b.balance), b.event_count);
                println!(
                    "  income:  {:>9}",
                    Amount::from_u64(b.total_income.unsigned_abs()),
                );
                println!(
                    "  expense: {:>9}",
                    Amount::from_u64(b.total_expense.unsigned_abs()),
                );
            }
            None => println!("No events found."),
        }
    }
}
