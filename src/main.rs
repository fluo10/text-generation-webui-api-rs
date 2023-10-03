
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    host: String,
    #[arg(short, long)]
    character_file: Option<PathBuf>,
    #[arg(short='n', long)]
    character_name: Option<String>,
    #[arg(short, long)]
    history: Option<String>,
}


#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let mut api = ChatApiBuilder::default();
    if let Some(x) = cli.character_file.clone() {
        todo!();
    } else if let Some(x) = cli.character_name.clone() {
        api = api.and_character_name(x);
    }

    if let Some(&x) = cli.history.as_ref() {
        api = api.and_history(x);
    }

    api.request(cli.host)

}
