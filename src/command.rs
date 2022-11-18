use std::io::{Write, Error};
use crate::validation::check_ascii;
use crate::text_detect::text_detect;
use std::process;
use crate::case::Case;
use crate::convert::convert_text;

pub fn run_convert(input: &str, from: Option<Case>, to: Case, output: &mut impl Write) -> Result<usize, Error> {
    check_ascii(&input);

    // TODO: unwrap_or?
    let from = match from {
        Some(it) => it,
        None => text_detect(&input)
            .main_case()
            .unwrap_or_else(|| {
                eprintln!("No case was detected in the provided input");
                process::exit(1);
            })
    };


    output.write(convert_text(&input, from, to).as_bytes())
}
