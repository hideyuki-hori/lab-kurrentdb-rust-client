mod app;
mod domain;
mod usecases;
mod infrastructure;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run().await
}
