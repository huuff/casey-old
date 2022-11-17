use casey::args::Args;
use clap::Parser;
use std::io;

fn main() {
    let args = Args::parse();

    casey::run(args, &mut io::stdout())
        .expect("Unexpected IO Error.");
}
