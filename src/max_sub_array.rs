pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .scan(-1, |s, x| {
            *s = if *s < 0 { x } else { *s + x };
            Some(*s)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod max_sub_array_tests {
    use super::*;

    #[test]
    fn max_sub_array_test_one() {
        // arrange
        let test = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

        // act
        let result = max_sub_array(test);

        // assert
        assert_eq!(result, 6);
    }
}
