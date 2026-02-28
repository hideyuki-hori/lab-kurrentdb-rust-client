use super::operation;

pub struct View;

impl super::Presenter for View {
    fn render(&self, input: &operation::Input, output: &operation::Output) {
        println!(
            "\u{2713} - {} \u{2192} {} [{}] (rev: {})",
            input.amount,
            input.account,
            input.category,
            output.revision,
        );
    }
}
