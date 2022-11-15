use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

// TODO: Literal inputs behind a flag? (--literal?)
// TODO: Short arg options
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
