pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .map(|x| match (x % 3 == 0, x % 5 == 0) {
            (true, true) => "FizzBuzz".into(),
            (true, false) => "Fizz".into(),
            (false, true) => "Buzz".into(),
            (false, false) => x.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod fizz_bizz_tests {
    use super::*;

    #[test]
    fn fizz_buzz_test_one() {
        // arrange
        let test = 15;

        // act
        let result = fizz_buzz(test);

        // assert
        assert_eq!(
            result,
            vec![
                "1".to_string(),
                "2".to_string(),
                "Fizz".to_string(),
                "4".to_string(),
                "Buzz".to_string(),
                "Fizz".to_string(),
                "7".to_string(),
                "8".to_string(),
                "Fizz".to_string(),
                "Buzz".to_string(),
                "11".to_string(),
                "Fizz".to_string(),
                "13".to_string(),
                "14".to_string(),
                "FizzBuzz".to_string()
            ]
        );
    }
}
