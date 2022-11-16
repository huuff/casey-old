use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Detect {
        #[arg(short, long)]
        inline: Option<String>,
    },
    Convert {
        #[arg(short, long)]
        inline: Option<String>,

        #[arg(short, long)]
        from: String,

        #[arg(short, long)]
        to: String,
    }
}
