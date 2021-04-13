pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut letters_one = word1.chars();
    let mut letters_two = word2.chars();

    let mut characters = vec![];

    for _ in 0..word1.len().max(word2.len()) {
        if let Some(c) = letters_one.next() {
            characters.push(c);
        }

        if let Some(c) = letters_two.next() {
            characters.push(c);
        }
    }

    characters.iter().collect::<String>()
}

#[cfg(test)]
mod merge_alternately_tests {
    use super::*;

    #[test]
    fn merge_alternately_test_one() {
        // arrange
        let word1 = "abc".to_string();
        let word2 = "pqr".to_string();

        // act
        let result = merge_alternately(word1, word2);

        // assert
        assert_eq!(result, "apbqcr".to_string());
    }
}
