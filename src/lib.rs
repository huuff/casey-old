pub mod case;
pub mod normalize;
pub mod convert;
pub mod args;
pub mod text_detect;

use crate::args::{Args, Command};
use crate::case::Case;
use crate::convert::{convert_token, convert_text};
use std::io;
use crate::text_detect::text_detect;
use std::process;

fn check_inline(input: &str) {
    if input.chars().any(|c| c.is_whitespace()) {
        println!("Sorry, the --inline option only accepts single tokens with no whitespace");
        process::exit(1);
    }
}

fn check_ascii(input: &str) {
    if input.chars().any(|c| !c.is_ascii()) {
        println!("Sorry, this program only accepts ASCII");
        process::exit(1);
    }
}

// TODO: Test
pub fn run(args: Args) -> String {
    let mut output = String::new();

    match args.command {
        Command::Detect { inline, verbose } => {
            if let Some(input) = inline {
                check_inline(&input);
                check_ascii(&input);

                let case = Case::detect(&input);
                
                if let Some(case) = case {
                    output.push_str(&case.to_string());
                }
            } else {
                let input = io::read_to_string(io::stdin()).unwrap();
                check_ascii(&input);

                let report = text_detect(&input);

                if verbose {
                    output.push_str(&report.long_description());
                } else {
                    output.push_str(&report.short_description());
                }
            }
        },
        Command::Convert { inline, from, to } => {
            // TODO: optional "from": detect the most
            // used case and do that one.
            let to = Case::parse(&to);

            match inline {
                Some(input) => {
                    check_inline(&input);
                    check_ascii(&input);

                    output.push_str(&convert_token(&input, &to));
                }
                None => {
                    let from = Case::parse(&from.unwrap());

                    let input = io::read_to_string(io::stdin()).unwrap();
                    check_ascii(&input);

                    output.push_str(&convert_text(&input, from, to));

                }
            }
        }
    };

    output
}
