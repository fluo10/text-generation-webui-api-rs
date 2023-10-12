use clap::Args; 

#[derive(Args)]
pub struct CommonArgs {
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short='n', long)]
    pub dry_run: bool,
}
