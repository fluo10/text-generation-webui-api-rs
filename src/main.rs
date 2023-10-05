use text_generation_webui_chat_rs::{Result, Cli};
use std::path::PathBuf;

use clap::Parser;



#[tokio::main]
async fn main() -> Result<()> {
    
   Cli::parse().run().await?;
    Ok(())
}
