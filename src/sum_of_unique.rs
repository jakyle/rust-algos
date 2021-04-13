pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut occurances_map = HashMap::new();

    for x in nums {
        *occurances_map.entry(x).or_insert(0) += 1;
    }

    occurances_map
        .iter()
        .filter(|(_, &v)| v == 1)
        .map(|(k, _)| k)
        .sum()
}

#[cfg(test)]
mod sum_of_unique_tests {
    use super::*;

    #[test]
    fn sum_of_unique_test_one() {
        // arrange
        let test = vec![1, 2, 3, 2];

        // act
        let result = sum_of_unique(test);

        // assert
        assert_eq!(result, 4);
    }
}
