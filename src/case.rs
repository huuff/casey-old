use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Case {
    Snake,
    ScreamingSnake, // Example: ENVIRONMENT_VAR
    Kebab,
    Camel,
    Pascal,
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
            if string.chars()
                .filter(|c| c.is_ascii_alphabetic())
                .all(|c| c.is_ascii_uppercase()) {
               return Some(Case::ScreamingSnake);
            }

            // If it has an uppercased letter, we know it's either camelCase or PascalCase
            // First letter decides
            let first_char = string.chars().nth(0).unwrap();
            return if first_char.is_ascii_uppercase() {
                Some(Case::Pascal)
            } else {
                Some(Case::Camel)
            };
        }

        if string.contains("_") {
            return Some(Case::Snake);
        } else if string.contains("-") {
            return Some(Case::Kebab);
        } else {
            // Must be a single lowercased word! Therefore we don't know
            // (snake? kebab? camel?)
            return None
        }
    }

    // TODO: This must be much more ergonomic to be useable, for exmaple:
    // * Offering short, two-word versions like ss, kc, cc, and pc
    // * Using just the first word like "snake", "kebab", "camel" or "pascal"
    pub fn parse(string: &str) -> Self {
        match string {
            "snake_case" => Case::Snake,
            "kebab-case" => Case::Kebab,
            "camelCase" => Case::Camel,
            "PascalCase" => Case::Pascal,
            _ => {
                panic!("{} not recognized!", string);
            }
        }
    }
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Case::Snake => "snake_case",
            Case::Kebab => "kebab-case",
            Case::Camel => "camelCase",
            Case::Pascal => "PascalCase",
            Case::ScreamingSnake => "SCREAMING_SNAKE",
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
        assert_eq!(case, Some(Case::Snake));
    }

    #[test]
    fn detects_screaming_snake() {
        // ARRANGE
        let word = "TEST_WORD";

        // ACT
        let case = Case::detect(word);

        // ASSERT
        assert_eq!(case, Some(Case::ScreamingSnake));
    }

    #[test]
    fn detects_kebab() {
        // ARRANGE
        let word = "test-word";

        // ACT
        let case = Case::detect(word);

        // ASSERT
        assert_eq!(case, Some(Case::Kebab));
    }

    #[test]
    fn detects_camel() {
        // ARRANGE
        let word = "testWord";

        // ACT
        let case = Case::detect(word);

        // ASSERT
        assert_eq!(case, Some(Case::Camel));
    }

    #[test]
    fn detects_pascal() {
        // ARRANGE
        let word = "TestWord";

        // ACT
        let case = Case::detect(word);

        // ASSERT
        assert_eq!(case, Some(Case::Pascal));
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
        assert_eq!(case, Some(Case::Pascal));
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
