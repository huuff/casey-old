use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Detect case")]
    Detect {
        #[arg(short, long, help = "Wether to show a full report of detected cases")]
        verbose: bool,

        #[arg(short, long, help = "Single token to detect")]
        inline: Option<String>,

        #[arg(short, long, help = "File on which to apply")]
        file: Option<String>,
    },
    #[command(about = "Convert case")]
    Convert {
        #[arg(short, long, help = "Single token to convert")]
        inline: Option<String>,

        #[arg(short, long, help = "File on which to apply")]
        file: Option<String>,

        #[arg(short, long, help = "Source case")]
        from: Option<String>,

        #[arg(short, long, help = "Destination case")]
        to: String,
    }
}
