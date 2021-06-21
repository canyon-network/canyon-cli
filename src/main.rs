pub mod app;
pub mod command;
pub mod runtime;
pub mod utils;

use anyhow::Result;

use self::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = App::init();
    cli.run().await?;

    Ok(())
}
