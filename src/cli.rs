use crate::{ChatApiBuilder, Character, History, Result};

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
    
}

impl Cli {
    #[tokio::main]
    pub async fn run(self) -> Result<()> {
        let cli = Cli::parse();

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
        
        tokio::task::spawn(async move {
           let request = api.request(&cli.host)?;
            println!("{:?}", request.text().await?);
            Ok(())
        }).await?
    }
    fn print_verobse(&self, s: &str) -> () {
        if self.verbose || self.dry_run {
            println!("{}", s)
        }
    }

}
