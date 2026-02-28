mod app;
mod domain;
mod usecases;
mod infrastructure;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    app::run().await
}
