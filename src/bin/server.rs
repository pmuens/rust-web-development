use rust_web_development::config;
use rust_web_development::{run, setup_store};

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenv::dotenv().ok();

    let config = config::Config::new().expect("Config can't be set");
    let store = setup_store(&config).await?;

    tracing::info!(
        "Q&A service build ID {}",
        env!("RUST_WEB_DEVELOPMENT_VERSION"),
    );

    run(config, store).await;

    Ok(())
}
