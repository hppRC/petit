use anyhow::Result;
use clap::Clap;
use petit::{application::Application, context::Context};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Application::parse();
    let ctx = Context::new().await?;

    app.run(ctx).await?;
    Ok(())
}
