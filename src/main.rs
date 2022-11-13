use casey::args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    let output = casey::run(args);

    println!("{}", output);
}
