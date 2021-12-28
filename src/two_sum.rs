pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    use std::cmp::Ordering;
    let (mut start, mut end) = (0, numbers.len() - 1);
    while start < end {
        match target.cmp(&(numbers[start] + numbers[end])) {
            Ordering::Less => end -= 1,
            Ordering::Equal => return vec![(start + 1) as i32, (end + 1) as i32],
            Ordering::Greater => start += 1,
        }
    }

    vec![-1, -1]
}

#[cfg(test)]
mod two_sum_tests {
    use super::*;

    #[test]
    fn two_sum_test_one() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }
}
