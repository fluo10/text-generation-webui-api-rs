use text_generation_webui_api::{ChatApiRequest, Character, History};
use super::common::CommonArgs;

use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

use clap::Args;
use anyhow::Result;

#[derive(Args)]
pub struct ChatArgs {
    host: String,
    input: String,
    #[arg(short, long)]
    character: Option<String>,
    #[arg(short='H', long)]
    history: Option<PathBuf>,
    

    #[arg(long)]
    chat_instruct_command: Option<String>,
    #[arg(long)]
    instruction_template: Option<String>,
    #[arg(long)]
    mode: Option<String>,

    #[command(flatten)]
    common: CommonArgs,
    
}

impl ChatArgs {
    pub async fn run(self) -> Result<()> {
        &self.print_verbose(&"start".to_string());

        println!("run");
        let mut api = ChatApiRequest::default().user_input(&self.input);
        if let Some(x) = self.character.as_ref() {
            if x.ends_with(".yaml") || x.ends_with("yml") {
                let data = fs::read_to_string(x)?;
                let character: Character = serde_yaml::from_str(&data)?;
                api = api.character(&character);
            } else {
                api = api.character_name(x);
            }
        }

        if let Some(x) = self.history.as_ref() {
            let data = fs::read_to_string(x)?;
            let history: History = serde_json::from_str(&data)?;
            api = api.history(&history);
        }
        if let Some(x) = self.chat_instruct_command.as_ref() {
            api.chat_instruct_command = Some(x.to_string());
        }
        if let Some(x) = self.mode.as_ref() {
            api.mode = Some(x.to_string());
        }
        if let Some(x) = self.instruction_template.as_ref() {
            api.instruction_template = Some(x.to_string());
        }
        
        let response = api.send(&self.host).await?;
        println!("{:?}", response);
        Ok(())
    }
    fn print_verbose(&self, s: &impl Display) -> () {
        if self.common.verbose || self.common.dry_run {
            println!("{}", s)
        }
    }

}
