use text_generation_webui_api::{ModelApiRequest, ModelApiResponse};
use super::common::CommonArgs;

use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

use clap::{Args, Subcommand};
use anyhow::Result;

#[derive(Args)]
pub struct ModelArgs {
    #[command(subcommand)]
    command: ModelCommand,
}

#[derive(Subcommand)]
enum ModelCommand {
    Info(ModelInfoArgs),
    List(ModelListArgs),
    Load(ModelLoadArgs),
}

impl ModelArgs {
    pub async fn run(self) -> Result<()> {
        match self.command {
            ModelCommand::Info(x) => x.run().await,
            ModelCommand::List(x) => x.run().await,
            ModelCommand::Load(x) => x.run().await,
        }
    }
}


#[derive(Args)]
pub struct ModelInfoArgs {
    #[command(flatten)]
    common: CommonArgs,
    host: String,
}

impl ModelInfoArgs {
    pub async fn run(self) -> Result<()> {
        let request = ModelApiRequest::info();
        let response = request.send(&self.host).await?;
        println!("{:?}", response);
        Ok(())
    }
}


#[derive(Args)]
pub struct ModelListArgs {
    #[command(flatten)]
    common: CommonArgs,
    host: String,
}

impl ModelListArgs {
    pub async fn run(self) -> Result<()> {
        let request = ModelApiRequest::list();
        let response = request.send(&self.host).await?;
        println!("{:?}", response);
        Ok(())
    }
}

#[derive(Args)]
pub struct ModelLoadArgs {
    #[command(flatten)]
    common: CommonArgs,
    host: String,
    model_name: String
}

impl ModelLoadArgs {
    pub async fn run(self) -> Result<()> {
        let request = ModelApiRequest::load(&self.model_name);
        let response = request.send(&self.host).await?;
        println!("{:?}", response);
        Ok(())
    }
}
