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
use std::fs;
use crate::validation::{check_inline, check_ascii, check_no_from};
use crate::command::{buffered_convert, buffered_detect};

fn choose_stream(file_name: Option<String>) -> Result<Box<dyn BufRead>, Error> {
    if let Some(file_name) = file_name {
        Ok(Box::new(BufReader::new(fs::File::open(file_name)?)))
    } else {
        Ok(Box::new(BufReader::new(io::stdin())))
    }
}

// TODO: Test
pub fn run(args: Args, output: &mut impl Write) -> Result<(), Error> {
    match args.command {
        Command::Detect { inline, file, verbose } => {
            if let Some(input) = inline {
                check_inline(&input);
                check_ascii(&input);

                let case = Case::detect(&input);

                if let Some(case) = case {
                    output.write(case.to_string().as_bytes())?
                } else {
                    output.write("Couldn't detect a case".as_bytes())?
                };
            } else {
                let input = choose_stream(file)?;

                let report = buffered_detect(input)?;

                if verbose {
                    output.write(report.long_description().as_bytes())?;
                } else {
                    output.write(report.short_description().as_bytes())?;
                };
            }
        },
        Command::Convert { inline, file, from, to } => {
            let to = Case::parse(&to);
            let from = from.and_then(|case| Case::detect(&case));

            if let Some(input) = inline {
                check_inline(&input);
                check_ascii(&input);
                check_no_from(&from);

                output.write(convert_token(&input, to).as_bytes())?;
            } else {
                let mut input = choose_stream(file)?;

                buffered_convert(&mut input, from, to, output)?;
            };
        }
    }
    Ok(())
}
