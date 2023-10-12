extern crate clap;
extern crate anyhow;
mod cli;


use cli::Cli;
use anyhow::Result;
use std::path::PathBuf;

use clap::Parser;



#[tokio::main]
async fn main() -> Result<()> {
    
   Cli::parse().run().await?;
    Ok(())
}
