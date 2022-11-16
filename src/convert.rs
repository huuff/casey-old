use crate::case::Case;
use crate::normalize::normalize;

enum SeparatorAction {
    Append(char),
    Uppercase,
}

pub fn convert_token(string: &str, case: &Case) -> String {
    let mut result = String::new();
    let separator_action = match case {
        Case::Snake | Case::ScreamingSnake => SeparatorAction::Append('_'),
        Case::Kebab => SeparatorAction::Append('-'),
        Case::Camel => SeparatorAction::Uppercase,
        Case::Pascal => SeparatorAction::Uppercase,
    };

    let normalized = normalize(string);

    let mut uppercase_next = *case == Case::ScreamingSnake || *case == Case::Pascal;
    for c in normalized.chars() {
        if c == ' ' {
            if let SeparatorAction::Append(separator) = separator_action {
                result.push(separator);
            } else {
                uppercase_next = true;
            }
        } else {
            if uppercase_next {
                result.push(c.to_ascii_uppercase());
                uppercase_next = false;
            } else {
                result.push(c);
            }
        }

        if *case == Case::ScreamingSnake {
            uppercase_next = true;
        }
    }

    result
}

// XXX: Likely not efficient
pub fn convert_text(text: &str, from_case: Case, to_case: Case) -> String {
    text.lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|token| {
                    let token_case = Case::detect(token);

                    if token_case.is_some() && token_case.unwrap() == from_case {
                        convert_token(token, &to_case)
                    } else {
                        String::from(token)
                    }

                })
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_token_snake_to_camel() {
        // ARRANGE
        let string = "test_word";

        // ACT
        let converted = convert_token(string, &Case::Camel);

        // ASSERT
        assert_eq!("testWord", converted);
    }

    #[test]
    fn converts_token_snake_to_kebab() {
        // ARRANGE
        let string = "test_word";

        // ACT
        let converted = convert_token(string, &Case::Kebab);

        // ASSERT
        assert_eq!("test-word", converted);
    }

    #[test]
    fn converts_token_snake_to_pascal() {
        // ARRANGE
        let string = "test_word";

        // ACT
        let converted = convert_token(string, &Case::Pascal);

        // ASSERT
        assert_eq!("TestWord", converted);
    }

    #[test]
    fn converts_token_long_camel_to_kebab() {
        // ARRANGE
        let string = "kindOfLongTestPhrase";

        // ACT
        let converted = convert_token(string, &Case::Kebab);

        // ASSERT
        assert_eq!("kind-of-long-test-phrase", converted);
    }

    #[test]
    fn converts_token_camel_case_to_screaming_snake() {
        // ARRANGE
        let string = "camelCase";

        // ACT
        let converted = convert_token(string, &Case::ScreamingSnake);

        // ASSERT
        assert_eq!("CAMEL_CASE", converted);
    }

    #[test]
    fn converts_text_camel_to_snake() {
        // ARRANGE
        let text = "\
This is a sampleText
with some camelCase
tokens";

        let expected = "\
This is a sample_text
with some camel_case
tokens";
        // ACT
        let converted = convert_text(text, Case::Camel, Case::Snake);

        // ASSERT
        assert_eq!(expected, converted);
    }
}
