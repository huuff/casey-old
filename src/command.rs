use std::io::{Write, Error, BufRead,};
use crate::validation::check_ascii;
use crate::text_detect::{text_detect, DetectReport};
use crate::case::Case;
use crate::convert::convert_text;
use std::process;

pub fn buffered_detect(input: Box<dyn BufRead>) -> Result<DetectReport, Error> {
    let mut report = DetectReport::new(); 

    for line in input.lines() {
        let line = line?;
        check_ascii(&line);
        text_detect(&line, &mut report);
    }

    Ok(report)
}

// XXX: Repetition for printing the lines... but nothing else
// would satisfy Rust
pub fn buffered_convert(input: &mut Box<dyn BufRead>, from: Option<Case>, to: Case, output: &mut impl Write) -> Result<(), Error> {
    match from {
        Some(from) => {

            for line in input.lines() {
                let line = line?;
                check_ascii(&line);

                output.write(convert_text(&line, from, to).as_bytes())?;
                output.write("\n".as_bytes())?;
            }
        },
        // If no source case is given, detect the most used one
        None => {
            eprintln!("Please note that converted from an auto-detected case requires buffering the whole input into memory and might be unfeasible with larger ones.\n");
            let mut input_string = String::new();
            input.read_to_string(&mut input_string)?;
            
            let mut case_report = DetectReport::new();
            text_detect(&input_string, &mut case_report);

            let from = case_report
                        .main_case()
                        .unwrap_or_else(|| {
                            eprintln!("No case was detected in the provided input");
                            process::exit(1);
                        });

            for line in input_string.lines() {
                check_ascii(&line);

                output.write(convert_text(&line, from, to).as_bytes())?;
                output.write("\n".as_bytes())?;
            }

        }
    }

    Ok(())
}


