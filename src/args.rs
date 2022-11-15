use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Detect {
        input: Option<String>,
    },
    Convert {
        input: Option<String>,

        #[arg(long)]
        from: String,

        #[arg(long)]
        to: String,
    }
}
