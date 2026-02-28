use crate::domain::query::ReceivedEvent;
use crate::domain::{AccountEvent, Amount};

fn format_balance(balance: i64) -> String {
    let abs = Amount::from_u64(balance.unsigned_abs());
    if balance < 0 {
        format!("-{}", abs)
    } else {
        format!("{}", abs)
    }
}

pub struct View;

impl super::Presenter for View {
    fn render_status(&self, balance: i64, event_count: usize) {
        println!(
            "current balance: {} ({} events)",
            format_balance(balance),
            event_count,
        );
        println!("watching... (Ctrl+C to stop)");
    }

    fn render_event(&self, received: &ReceivedEvent, balance: i64) {
        let line = match &received.event {
            AccountEvent::Income {
                amount,
                description,
            } => {
                format!(
                    "[{}:{}] +{} {} \u{2192} {}",
                    received.hours,
                    received.minutes,
                    amount,
                    description,
                    format_balance(balance),
                )
            }
            AccountEvent::Expense {
                amount,
                category,
                description,
            } => {
                format!(
                    "[{}:{}] -{} [{}] {} \u{2192} {}",
                    received.hours,
                    received.minutes,
                    amount,
                    category,
                    description,
                    format_balance(balance),
                )
            }
        };

        println!("{}", line);
    }
}
