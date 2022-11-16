use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

// TODO: Short arg options
#[derive(Subcommand)]
pub enum Command {
    Detect {
        #[arg(long)]
        inline: Option<String>,
    },
    Convert {
        #[arg(long)]
        inline: Option<String>,

        #[arg(long)]
        from: String,

        #[arg(long)]
        to: String,
    }
}
