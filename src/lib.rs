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
use std::fs;

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

fn check_no_from(from: Option<String>) {
    if from.is_some() {
        println!("The --from flag is not allowed for this operation");
        process::exit(1);
    }
}

fn detect_and_collect_report(input: &str, verbose: bool, output: &mut String) {
    check_ascii(&input);

    let report = text_detect(&input);

    if verbose {
        output.push_str(&report.long_description());
    } else {
        output.push_str(&report.short_description());
    }
}

// TODO: Test
pub fn run(args: Args) -> String {
    let mut output = String::new();

    match args.command {
        Command::Detect { inline, file, verbose } => {
            if let Some(input) = inline {
                check_inline(&input);
                check_ascii(&input);

                let case = Case::detect(&input);
                
                if let Some(case) = case {
                    output.push_str(&case.to_string());
                }
            } else if let Some(file) = file {
                let input = fs::read_to_string(file)
                    .expect("Failed to open file"); 

                detect_and_collect_report(&input, verbose, &mut output);

            } else {
                let input = io::read_to_string(io::stdin()).unwrap();
                detect_and_collect_report(&input, verbose, &mut output);
            }
        },
        Command::Convert { inline, from, to } => {
            let to = Case::parse(&to);

            match inline {
                Some(input) => {
                    check_inline(&input);
                    check_ascii(&input);
                    check_no_from(from);

                    output.push_str(&convert_token(&input, &to));
                }
                None => {
                    let input = io::read_to_string(io::stdin()).unwrap();
                    let from = match from {
                        Some(it) => Case::parse(&it),
                        None => text_detect(&input)
                            .main_case()
                            .unwrap_or_else(|| {
                                println!("No case was detected in the provided input");
                                process::exit(1);
                            })
                    };

                    check_ascii(&input);

                    output.push_str(&convert_text(&input, from, to));

                }
            }
        }
    };

    output
}
