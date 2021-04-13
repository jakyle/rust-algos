pub fn is_power_of_four(num: i32) -> bool {
    num > 0 && num & (num - 1) == 0 && (num as u32 & 0xAAAAAAAA) == 0
}

#[cfg(test)]
mod is_power_of_four_tests {
    use super::*;

    #[test]
    fn is_power_of_four_test_one() {
        // arrange
        let num_test = 16;

        // act
        let result = is_power_of_four(num_test);

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn is_power_of_four_test_two() {
        // arrange
        let num_test = 256;

        // act
        let result = is_power_of_four(num_test);

        // assert
        assert_eq!(result, true);
    }
}
