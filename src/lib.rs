pub mod case;
pub mod token;
pub mod normalize;
pub mod convert;
pub mod args;
pub mod text_detect;
pub mod validation;
pub mod command;

use crate::args::{Args, Command};
use crate::case::Case;
use crate::convert::convert_token;
use std::io::{self, Write, Error, BufReader, BufRead};
use crate::text_detect::text_detect;
use std::fs;
use crate::validation::{check_inline, check_ascii, check_no_from};
use crate::command::run_convert;

fn detect_and_collect_report(input: &str, verbose: bool, output: &mut impl Write) -> Result<usize, Error> {
    check_ascii(&input);

    let report = text_detect(&input);

    if verbose {
        output.write(report.long_description().as_bytes())
    } else {
        output.write(report.short_description().as_bytes())
    }
}


// TODO: Test
// TODO: Use input with an abstraction, as for output (i.e., use std::io::Read)
// TODO: Better result type (Result<(), Error>)
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
            let to = Case::parse(&to);
            let from = from.and_then(|case| Case::detect(&case));

            if let Some(input) = inline {
                check_inline(&input);
                check_ascii(&input);
                check_no_from(&from);

                output.write(convert_token(&input, &to).as_bytes())
            } else {
                let input: Box<dyn BufRead> = if let Some(file) = file {
                    Box::new(BufReader::new(fs::File::open(file)?))
                } else {
                    Box::new(BufReader::new(io::stdin()))
                };
                let input = BufReader::new(input);

                for line in input.lines() {
                    run_convert(&line?, from, to, output)?;
                    output.write("\n".as_bytes())?;
                }
                
                Ok(0)

            } 
        }
    }
}
