pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    use std::collections::{HashMap, HashSet};

    let mut occurances_map: HashMap<i32, i32> = HashMap::new();
    for num in arr {
        *occurances_map.entry(num).or_insert(0) += 1;
    }
    let mut uniq = HashSet::new();
    occurances_map.values().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod unique_occurrences_tests {
    use super::*;

    #[test]
    fn unique_occurrences_test_one() {

        // arrange
        let test = vec![1,2,2,1,1,3];

        // act
        let result = unique_occurrences(test);

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn unique_occurrences_test_two() {

        // arrange
        let test = vec![-3,0,1,-3,1,1,1,-3,10,0];

        // act
        let result = unique_occurrences(test);

        // assert
        assert_eq!(result, true);
    }

    #[test]
    fn unique_occurrences_test_three() {

        // arrange
        let test = vec![1, 2];

        // act
        let result = unique_occurrences(test);

        // assert
        assert_eq!(result, false);
    }
}