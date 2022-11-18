use std::process;
use crate::case::Case;


pub fn check_inline(input: &str) {
    if input.chars().any(|c| c.is_whitespace()) {
        eprintln!("Sorry, the --inline option only accepts single tokens with no whitespace");
        process::exit(1);
    }
}

pub fn check_ascii(input: &str) {
    if input.chars().any(|c| !c.is_ascii()) {
        eprintln!("Sorry, this program only accepts ASCII");
        process::exit(1);
    }
}

pub fn check_no_from(from: &Option<Case>) {
    if from.is_some() {
        eprintln!("The --from flag is not allowed for this operation");
        process::exit(1);
    }
}
