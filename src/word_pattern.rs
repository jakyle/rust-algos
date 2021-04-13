pub fn word_pattern(pattern: String, s: String) -> bool {
    use std::collections::HashMap;

    if pattern.len() != s.split_whitespace().count() {
        return false;
    }

    let pattern_word: HashMap<char, &str> = pattern.chars().zip(s.split_whitespace()).collect();
    let word_pattern: HashMap<&str, char> = pattern_word.iter().map(|(&k, &v)| (v, k)).collect();

    for (pattern, word) in pattern.chars().zip(s.split_whitespace()) {
        match (word_pattern.get(word), pattern_word.get(&pattern)) {
            (Some(&a), Some(&b)) => {
                if a != pattern || b != word {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod word_pattern_tests {
    use super::*;

    #[test]
    fn word_pattern_test_one() {
        // arrange
        let pattern = "abba".into();
        let s = "dog cat cat dog".into();

        // act
        let result = word_pattern(pattern, s);

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn word_pattern_test_two() {
        // arrange
        let pattern = "abba".into();
        let s = "dog dog dog dog".into();

        // act
        let result = word_pattern(pattern, s);

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn word_pattern_test_three() {
        // arrange
        let pattern = "abc".into();
        let s = "dog cat dog".into();

        // act
        let result = word_pattern(pattern, s);

        // assert
        assert_eq!(result, false);
    }
}
