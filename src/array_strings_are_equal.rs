/// Given two string arrays word1 and word2, return true if the two arrays represent the same string, and false otherwise.
///
/// A string is represented by an array if the array elements concatenated in order forms the string.
///
///
///
/// Example 1:
///
/// Input: word1 = ["ab", "c"], word2 = ["a", "bc"]
/// Output: true
/// Explanation:
/// word1 represents string "ab" + "c" -> "abc"
/// word2 represents string "a" + "bc" -> "abc"
/// The strings are the same, so return true.
///
/// Example 2:
///
/// Input: word1 = ["a", "cb"], word2 = ["ab", "c"]
/// Output: false
///
/// Example 3:
///
/// Input: word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]
/// Output: true
///
///
///
/// Constraints:
///
///     1 <= word1.length, word2.length <= 103
///     1 <= word1[i].length, word2[i].length <= 103
///     1 <= sum(word1[i].length), sum(word2[i].length) <= 103
///     word1[i] and word2[i] consist of lowercase letters.
pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    word1.concat() == word2.concat()
}

#[cfg(test)]
mod array_strings_are_equal_tests {
    use super::*;

    #[test]
    pub fn array_strings_are_equal_test_one() {
        // arrange
        let word_1 = vec!["ab".to_string(), "c".to_string()];
        let word_2 = vec!["a".to_string(), "bc".to_string()];

        // act
        let result = array_strings_are_equal(word_1, word_2);

        // assert
        assert!(result, true);
    }
}
