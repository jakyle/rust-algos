pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    (1..=arr.len())
        .step_by(2)
        .map(|x| arr.windows(x).map(|y| y.iter().sum::<i32>()).sum::<i32>())
        .sum()
}

#[cfg(test)]
mod sum_odd_length_subarrays_tests {
    use super::*;

    #[test]
    fn sum_odd_length_subarrays_one() {
        // arrange
        let test = vec![1, 4, 2, 5, 3];

        // act
        let result = sum_odd_length_subarrays(test);

        // assert
        assert_eq!(result, 58);
    }
}
