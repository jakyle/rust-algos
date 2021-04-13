pub fn maximum69_number(num: i32) -> i32 {
    num.to_string().replacen("6", "9", 1).parse().unwrap()
}

#[cfg(test)]
mod maximum69_number_tests {
    use super::*;

    #[test]
    fn maximum69_number_test_one() {
        // arrange
        let test = 9669;

        // act
        let result = maximum69_number(test);

        // assert
        assert_eq!(result, 9969);
    }
}
