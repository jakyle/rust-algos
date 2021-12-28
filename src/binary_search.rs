pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    use std::cmp::Ordering;
    let (mut low, mut high) = (0, nums.len());
    while low < high {
        let mid = low + (high - low) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Less => low = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => high = mid,
        }
    }

    None
}

#[cfg(test)]
mod binary_search_tests {
    use super::*;

    #[test]
    fn binary_test_one() {
        let numbers = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(binary_search(&numbers, 3), Some(2));
    }
}
