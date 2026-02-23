pub mod alert_watch;
pub mod balance;
pub mod expense;
pub mod history;
pub mod income;
pub mod projection_setup;
pub mod projection_status;
pub mod stats_category;
pub mod stats_summary;
pub mod watch;

pub fn format_amount(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();

    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }

    return result.chars().rev().collect();
}

pub fn format_amount_f64(n: f64) -> String {
    let abs = n.abs();
    let integer = abs as u64;
    let formatted = format_amount(integer);
    if n < 0.0 {
        format!("-{}", formatted)
    } else {
        formatted
    }
}
