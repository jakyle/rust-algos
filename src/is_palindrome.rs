pub fn is_palindrome(s: String) -> bool {
    let s = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    s.chars().zip(s.chars().rev()).all(|(a, b)| a == b)
}

#[cfg(test)]
mod is_palindrome_tests {
    use super::*;

    #[test]
    fn is_palindrome_test_one() {
        // arrange
        let test = "A man, a plan, a canal: Panama".into();

        // act
        let result = is_palindrome(test);

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn is_palindrome_test_two() {
        // arrange
        let test = "race a car".into();

        // act
        let result = is_palindrome(test);

        // assert
        assert_eq!(result, false);
    }
}
