pub fn single_number(nums: Vec<i32>) -> i32 {
    // nums.iter().fold(0, |acc, x| acc ^ x)

    let mut result = 0;
    for num in nums {
        let x = result ^ num;
        result = x;
    }

    result
}

#[cfg(test)]
mod single_number_tests {
    use super::*;

    #[test]
    fn single_number_test_one() {
        // arrange
        let test = vec![5, 4, 3, 2, 1, 2, 3, 4, 5];

        // act
        let result = single_number(test);

        // assert
        assert_eq!(result, 1);
    }
}
