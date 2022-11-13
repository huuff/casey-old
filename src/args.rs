use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Detect {
        input: String,        
    },
    Convert {
        input: String,
        #[arg(long)]
        to: String,
    }
}
