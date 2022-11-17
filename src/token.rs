use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TOKEN_CHAR_REGEX: Regex = Regex::new(r"^[a-zA-Z_-]$").unwrap();
}

pub fn is_token_char(character: char) -> bool {
    // XXX: Array to store bytes so we can encode all characters
    // without heap allocations
    // https://stackoverflow.com/a/47634755/15768984
    let mut string_bytes_aux = [0; 4];
    TOKEN_CHAR_REGEX.is_match(
        character.encode_utf8(&mut string_bytes_aux)
    )
}

pub fn is_token(string: &str) -> bool {
    string.chars().all(is_token_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_token_char_dash() {
        assert_eq!(true, is_token_char('-'))
    }

    #[test]
    fn detects_token_char_uppercase() {
        assert_eq!(true, is_token_char('Z'))
    }

    #[test]
    fn detects_non_token_char() {
        assert_eq!(false, is_token_char('*'))
    }

    #[test]
    fn detects_token() {
        assert_eq!(true, is_token("snake-case"));
    }

    #[test]
    fn detects_non_token() {
        assert_eq!(false, is_token("snake//case"));
    }
}
