pub mod case;
pub mod token;
pub mod normalize;
pub mod convert;
pub mod args;
pub mod text_detect;

use crate::args::{Args, Command};
use crate::case::Case;
use crate::convert::{convert_token, convert_text};
use std::io::{self, Write, Error};
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

fn detect_and_collect_report(input: &str, verbose: bool, output: &mut impl Write) -> Result<usize, Error> {
    check_ascii(&input);

    let report = text_detect(&input);

    if verbose {
        output.write(report.long_description().as_bytes())
    } else {
        output.write(report.short_description().as_bytes())
    }
}

fn convert_and_collect_output(input: &str, from: Option<String>, to: String, output: &mut impl Write) -> Result<usize, Error> {
    check_ascii(&input);

    let to = Case::parse(&to);
    let from = match from {
        Some(it) => Case::parse(&it),
        None => text_detect(&input)
            .main_case()
            .unwrap_or_else(|| {
                println!("No case was detected in the provided input");
                process::exit(1);
            })
    };


    output.write(convert_text(&input, from, to).as_bytes())
}

// TODO: Test
// TODO: Use input with an abstraction, as for output (i.e., use std::io::Read)
pub fn run(args: Args, output: &mut impl Write) -> Result<usize, Error> {
    match args.command {
        Command::Detect { inline, file, verbose } => {
            if let Some(input) = inline {
                check_inline(&input);
                check_ascii(&input);

                let case = Case::detect(&input);

                if let Some(case) = case {
                    output.write(case.to_string().as_bytes())
                } else {
                    output.write("Couldn't detect a case".as_bytes())
                }
            } else if let Some(file) = file {
                let input = fs::read_to_string(file)
                    .expect("Failed to open file"); 

                detect_and_collect_report(&input, verbose, output)

            } else {
                let input = io::read_to_string(io::stdin()).unwrap();
                detect_and_collect_report(&input, verbose, output)
            }
        },
        Command::Convert { inline, file, from, to } => {
            if let Some(input) = inline {
                check_inline(&input);
                check_ascii(&input);
                check_no_from(from);
                let to = Case::parse(&to);

                output.write(convert_token(&input, &to).as_bytes())
            } else if let Some(file) = file {
                let input = fs::read_to_string(file)
                            .expect("Couldn't read file");
                convert_and_collect_output(&input, from, to, output)

            } else {
                let input = io::read_to_string(io::stdin()).unwrap();
                convert_and_collect_output(&input, from, to, output)

            }
        }
    }
}
