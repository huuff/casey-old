pub mod case;
pub mod normalize;
pub mod convert;
pub mod args;

use crate::args::{Args, Command};
use crate::case::Case;
use crate::convert::convert_text;
use std::io;

// TODO: Test
pub fn run(args: Args) -> String {
    let mut output = String::new();

    match args.command {
        Command::Detect { input } => {
            // TODO: Handle missing input
            let case =  Case::detect(&input.unwrap());
            
            if let Some(variant) = case {
                output.push_str(&variant.to_string());
            }
        },
        Command::Convert { input, from, to } => {
            let from = Case::parse(&from);
            let to = Case::parse(&to);

            match input {
                Some(input) => {
                    output.push_str(&convert_text(&input, from, to));
                }
                None => {
                    // TODO: Optional from here? (And just detect it)
                    let input = io::read_to_string(io::stdin()).unwrap();

                    output.push_str(&convert_text(&input, from, to));

                }
            }
        }
    };

    output
}
