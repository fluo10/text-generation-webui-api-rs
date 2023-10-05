use crate::{ChatApiBuilder, Character, History, Result};


use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    host: String,
    input: String,
    #[arg(short, long)]
    character: Option<String>,
    #[arg(short='H', long)]
    history: Option<PathBuf>,
    #[arg(short, long)]
    verbose: bool,
    #[arg(short='n', long)]
    dry_run: bool,

    #[arg(long)]
    chat_instruct_command: Option<String>,
    #[arg(long)]
    instruction_template: Option<String>,
    #[arg(long)]
    mode: Option<String>,
    
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let cli = Cli::parse();
        &self.print_verbose(&"start".to_string());

        println!("run");
        let mut api = ChatApiBuilder::default().user_input(&self.input);
        if let Some(x) = cli.character.as_ref() {
            if x.ends_with(".yaml") || x.ends_with("yml") {
                let data = fs::read_to_string(x)?;
                let character: Character = serde_yaml::from_str(&data)?;
                api = api.character(&character);
            } else {
                api = api.character_name(x);
            }
        }

        if let Some(x) = cli.history.as_ref() {
            let data = fs::read_to_string(x)?;
            let history: History = serde_json::from_str(&data)?;
            api = api.history(&history);
        }
        if let Some(x) = cli.chat_instruct_command.as_ref() {
            api.chat_instruct_command = Some(x.to_string());
        }
        if let Some(x) = cli.mode.as_ref() {
            api.mode = Some(x.to_string());
        }
        if let Some(x) = cli.instruction_template.as_ref() {
            api.instruction_template = Some(x.to_string());
        }
        
        let request = api.request(&cli.host).await?;
        println!("{:?}", request.text().await?);
        Ok(())
    }
    fn print_verbose(&self, s: &impl Display) -> () {
        if self.verbose || self.dry_run {
            println!("{}", s)
        }
    }

}
