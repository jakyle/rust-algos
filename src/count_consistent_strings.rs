pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut max = 0;
    let allowed_letters = allowed.chars().collect::<Vec<char>>();

    use std::collections::HashSet;
    for word in words {
        let mut letters: HashSet<char> = HashSet::new();

        for letter in word.chars() {
            letters.insert(letter);
        }

        let mut is_match = true;
        for letter in letters {
            if !allowed_letters.contains(&letter) {
                is_match = false;
                break;
            }
        }

        if is_match {
            max += 1;
        }
    }

    max
}

#[cfg(test)]
mod common_chars_tests {
    use super::*;

    #[test]
    fn common_chars_test1() {
        // arrange
        let input_one = "ab".to_string();

        let input_two = vec![
            "ad".to_string(),
            "bd".to_string(),
            "aaab".to_string(),
            "baa".to_string(),
            "badab".to_string(),
        ];

        // act
        let result = count_consistent_strings(input_one, input_two);

        // assert
        assert_eq!(result, 2);
    }
}
