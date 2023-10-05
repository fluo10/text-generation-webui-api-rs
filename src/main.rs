use text_generation_webui_chat_rs::{Result, Cli};
use std::path::PathBuf;

use clap::Parser;



#[tokio::main]
async fn main() -> Result<()> {
    tokio::task::spawn(async{
      Cli::parse().run()
   }).await?
}
