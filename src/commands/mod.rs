pub mod balance;
pub mod expense;
pub mod history;
pub mod income;
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
