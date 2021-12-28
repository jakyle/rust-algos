/// Given two strings a and b, return true if you can swap two letters in a so the result is equal to b, otherwise, return false.
///
/// Swapping letters is defined as taking two indices i and j (0-indexed) such that i != j and
/// swapping the characters at a[i] and b[j]. For example, swapping at indices 0 and 2 in "abcd" results in "cbad".
pub fn buddy_strings(a: String, b: String) -> bool {
    let a_chars = a.chars().collect::<Vec<char>>();
    let b_chars = b.chars().collect::<Vec<char>>();
    let len = a_chars.len();

    for i in 0..len {
        for j in i + 1..len {}
    }

    true
}

#[cfg(test)]
mod buddy_strings_tests {
    use super::*;

    #[test]
    fn buddy_strings_tests_one() {
        // arrange
        let a = "ab".to_string();
        let b = "ba".to_string();

        // act
        let result = buddy_strings(a, b);

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn buddy_strings_tests_two() {
        // arrange
        let a = "ab".to_string();
        let b = "ab".to_string();

        // act
        let result = buddy_strings(a, b);

        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn buddy_strings_tests_four() {
        // arrange
        let a = "aaaaaaabc".to_string();
        let b = "aaaaaaacb".to_string();

        // act
        let result = buddy_strings(a, b);

        // assert
        assert_eq!(result, true);
    }
}
