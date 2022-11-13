use crate::case::Case;
use crate::normalize::normalize;

enum SeparatorAction {
    Append(char),
    Uppercase,
}

pub fn convert(string: &str, case: Case) -> String {
    let mut result = String::new();
    let separator_action = match case {
        Case::SNAKE => SeparatorAction::Append('_'),
        Case::KEBAB => SeparatorAction::Append('-'),
        Case::CAMEL => SeparatorAction::Uppercase,
        Case::PASCAL => SeparatorAction::Uppercase,
    };

    let normalized = normalize(string);

    let mut uppercase_next = false;
    for (i, c) in normalized.chars().enumerate() {
        if i == 0 && case == Case::PASCAL {
            result.push(c.to_ascii_uppercase());
        } else if c == ' ' {
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
        let converted = convert(string, Case::CAMEL);

        // ASSERT
        assert_eq!(String::from("testWord"), converted);
    }

    #[test]
    fn converts_snake_to_kebab() {
        // ARRANGE
        let string = "test_word";

        // ACT
        let converted = convert(string, Case::KEBAB);

        // ASSERT
        assert_eq!(String::from("test-word"), converted);
    }

    #[test]
    fn converts_snake_to_pascal() {
        // ARRANGE
        let string = "test_word";

        // ACT
        let converted = convert(string, Case::PASCAL);

        // ASSERT
        assert_eq!(String::from("TestWord"), converted);
    }
}
