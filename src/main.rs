use text_generation_webui_api::{Result, Cli};
use std::path::PathBuf;

use clap::Parser;



#[tokio::main]
async fn main() -> Result<()> {
    
   Cli::parse().run().await?;
    Ok(())
}
