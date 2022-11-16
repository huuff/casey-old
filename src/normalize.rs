pub fn normalize(string: &str) -> String {
    let mut result = String::new(); 

    for (i, c) in string.chars().enumerate() {
        if i != 0 && (c == '_' || c == '-' || c.is_ascii_uppercase())  {
            result.push(' ');
        }

        if c.is_ascii_alphabetic() {
            result.push(c.to_ascii_lowercase());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_camel_case() {
        // ARRANGE
        let string = "testWord";

        // ACT
        let normalized = normalize(string);

        // ASSERT
        assert_eq!("test word", normalized);
    }

    #[test]
    fn normalizes_pascal_case() {
        // ARRANGE
        let string = "TestWord";

        // ACT
        let normalized = normalize(string);

        // ASSERT
        assert_eq!("test word", normalized);
    }

    #[test]
    fn normalizes_kebab_case() {
        // ARRANGE
        let string = "test-word";

        // ACT
        let normalized = normalize(string);

        // ASSERT
        assert_eq!("test word", normalized);
    }

    #[test]
    fn normalizes_snake_case() {
        // ARRANGE
        let string = "test_word";

        // ACT
        let normalized = normalize(string);

        // ASSERT
        assert_eq!("test word", normalized);
    }

    #[test]
    fn leaves_single_lowercase_word_intact() {
        // ARRANGE
        let string = "testword";

        // ACT
        let normalized = normalize(string);

        // ASSERT
        assert_eq!("testword", normalized);
    }
}
