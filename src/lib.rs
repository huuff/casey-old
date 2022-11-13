pub mod case;
pub mod normalize;
pub mod convert;
pub mod args;

use crate::args::{Args, Command};
use crate::case::Case;

pub fn run(args: Args) -> String {
    let mut output = String::new();

    match args.command {
        Command::Detect { input } => {
            let case =  Case::detect(&input);
            
            if let Some(variant) = case {
                output.push_str(&variant.to_string());
            }
        } 
    };

    output
}
