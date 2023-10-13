mod common;
mod chat;
mod model;

use common::CommonArgs;
use chat::ChatArgs;
use model::ModelArgs;

use anyhow::Result;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Chat(ChatArgs),
    Model(ModelArgs),
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Command::Chat(x) => x.run().await,
            Command::Model(x) => x.run().await
        }
    }
}
