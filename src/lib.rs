pub mod case;
pub mod normalize;
pub mod convert;
pub mod args;
pub mod text_detect;

use crate::args::{Args, Command};
use crate::case::Case;
use crate::convert::convert_text;
use std::io;
use crate::text_detect::text_detect;

// TODO: Test
pub fn run(args: Args) -> String {
    let mut output = String::new();

    match args.command {
        Command::Detect { input } => {
            if let Some(input) = input {
                // TODO: Fail somehow if string has whitespace
                let case = Case::detect(&input);
                
                if let Some(case) = case {
                    output.push_str(&case.to_string());
                }
            } else {
                let input = io::read_to_string(io::stdin()).unwrap();

                let report = text_detect(&input);

                output.push_str(&report.to_string());
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
                    // Wtf does the below comment mean?
                    // TODO: Optional from here? (And just detect it)
                    let input = io::read_to_string(io::stdin()).unwrap();

                    output.push_str(&convert_text(&input, from, to));

                }
            }
        }
    };

    output
}
