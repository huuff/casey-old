use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Case {
    SNAKE,
    KEBAB,
    CAMEL,
    PASCAL,
}

impl Case {
    // TODO: do this all with regex
    pub fn detect(string: &str) -> Option<Case> {
        if string.is_empty() {
            panic!("Can't detect case on an empty string!");
        }

        if string.contains(char::is_whitespace) {
            panic!("Can't detect on a string with whitespace!");
        }

        if string.contains(char::is_uppercase) {
            // If it has an uppercased letter, we know it's either camelCase or PascalCase
            // First letter decides
            let first_char = string.chars().nth(0).unwrap();
            return if first_char.is_uppercase() {
                Some(Case::PASCAL)
            } else {
                Some(Case::CAMEL)
            };
        }

        if string.contains("_") {
            return Some(Case::SNAKE);
        } else if string.contains("-") {
            return Some(Case::KEBAB);
        } else {
            // Must be a single lowercased word! Therefore we don't know
            // (snake? kebab? camel?)
            return None
        }
    }
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Case::SNAKE => "snake_case",
            Case::KEBAB => "kebab-case",
            Case::CAMEL => "camelCase",
            Case::PASCAL => "PascalCase",
        };
    
        write!(f, "{}", description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn detects_snake() {
        // ARRANGE
        let word = "test_word";

        // ACT
        let case = Case::detect(word);

        // ASSERT
        assert_eq!(case, Some(Case::SNAKE));
    }

    #[test]
    fn detects_kebab() {
        // ARRANGE
        let word = "test-word";

        // ACT
        let case = Case::detect(word);

        // ASSERT
        assert_eq!(case, Some(Case::KEBAB));
    }

    #[test]
    fn detects_camel() {
        // ARRANGE
        let word = "testWord";

        // ACT
        let case = Case::detect(word);

        // ASSERT
        assert_eq!(case, Some(Case::CAMEL));
    }

    #[test]
    fn detects_pascal() {
        // ARRANGE
        let word = "TestWord";

        // ACT
        let case = Case::detect(word);

        // ASSERT
        assert_eq!(case, Some(Case::PASCAL));
    }

    #[test]
    fn cant_detect_single_lowercase_word() {
        // ARRANGE
        let word = "test";

        // ACT
        let case = Case::detect(word);

        // ASSERT
        assert_eq!(case, None);
    }

    #[test]
    fn detects_single_word_pascal() {
        // ARRANGE
        let word = "Test";

        // ACT
        let case = Case::detect(word);

        // ASSERT
        assert_eq!(case, Some(Case::PASCAL));
    }

    #[test]
    #[should_panic(expected = "empty string")]
    fn panics_on_empty_string() {
        // ARRANGE
        let word = "";

        // ACT
        Case::detect(word);
    }

    #[test]
    #[should_panic(expected = "whitespace")]
    fn panics_on_whitespace() {
        // ARRANGE
        let word = "test word";

        // ACT
        Case::detect(word);
    }
}
