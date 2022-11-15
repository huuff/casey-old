use crate::case::Case;
use crate::normalize::normalize;

enum SeparatorAction {
    Append(char),
    Uppercase,
}

pub fn convert(string: &str, case: Case) -> String {
    let mut result = String::new();
    let separator_action = match case {
        Case::Snake | Case::ScreamingSnake => SeparatorAction::Append('_'),
        Case::Kebab => SeparatorAction::Append('-'),
        Case::Camel => SeparatorAction::Uppercase,
        Case::Pascal => SeparatorAction::Uppercase,
    };

    let normalized = normalize(string);

    let mut uppercase_next = case == Case::ScreamingSnake || case == Case::Pascal;
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

        if case == Case::ScreamingSnake {
            uppercase_next = true;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_snake_to_camel() {
        // ARRANGE
        let string = "test_word";

        // ACT
        let converted = convert(string, Case::Camel);

        // ASSERT
        assert_eq!(String::from("testWord"), converted);
    }

    #[test]
    fn converts_snake_to_kebab() {
        // ARRANGE
        let string = "test_word";

        // ACT
        let converted = convert(string, Case::Kebab);

        // ASSERT
        assert_eq!(String::from("test-word"), converted);
    }

    #[test]
    fn converts_snake_to_pascal() {
        // ARRANGE
        let string = "test_word";

        // ACT
        let converted = convert(string, Case::Pascal);

        // ASSERT
        assert_eq!(String::from("TestWord"), converted);
    }

    #[test]
    fn converts_long_camel_to_kebab() {
        // ARRANGE
        let string = "kindOfLongTestPhrase";

        // ACT
        let converted = convert(string, Case::Kebab);

        // ASSERT
        assert_eq!(String::from("kind-of-long-test-phrase"), converted);
    }

    #[test]
    fn converts_camel_case_to_screaming_snake() {
        // ARRANGE
        let string = "camelCase";

        // ACT
        let converted = convert(string, Case::ScreamingSnake);

        // ASSERT
        assert_eq!(String::from("CAMEL_CASE"), converted);
    }
}
