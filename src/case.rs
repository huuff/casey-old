
#[derive(Debug, PartialEq)]
pub enum Case {
    SNAKE,
    KEBAB,
    CAMEL,
    PASCAL,
}

impl Case {
    // TODO: Panic when string contains a space
    // TODO: Panic on empty string
    pub fn detect(string: &str) -> Option<Case> {
        if string.chars().any(|c| c.is_uppercase()) {
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
}
