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

pub fn buffered_convert(input: Box<dyn BufRead>, from: Option<Case>, to: Case, output: &mut impl Write) -> Result<usize, Error> {
    // TODO: unwrap_or?
    // If no source case is given, detect the most used one
    // TODO: Not working, need to buffer everything in memory
    //let from = match from {
        //Some(it) => it,
        //None => buffered_detect(input)?
                //.main_case()
                //.unwrap_or_else(|| {
                    //eprintln!("No case was detected in the provided input");
                    //process::exit(1);
                //})
    //};
    
    let from = from.unwrap_or(Case::Snake);

    for line in input.lines() {
        let line = line?;
        check_ascii(&line);

        output.write(convert_text(&line, from, to).as_bytes())?;
    }

    Ok(0)
}


