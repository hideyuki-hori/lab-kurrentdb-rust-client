use crate::domain::query::HistoryEntry;
use crate::domain::AccountEvent;

pub struct View;

impl super::Presenter for View {
    fn render(&self, entries: &[HistoryEntry]) {
        if entries.is_empty() {
            println!("No events found.");
            return;
        }

        for entry in entries {
            let (sign, amt, desc) = match &entry.event {
                AccountEvent::Income {
                    amount,
                    description,
                } => ("+", format!("{}", amount), format!("{}", description)),
                AccountEvent::Expense {
                    amount,
                    category,
                    description,
                } => {
                    let desc = format!("[{}] {}", category, description);
                    ("-", format!("{}", amount), desc)
                }
            };

            println!(
                "#{:<3} {}{:>9} {:<30} {}",
                entry.revision,
                sign,
                amt,
                desc,
                entry.timestamp,
            );
        }
    }
}
