pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

#[cfg(test)]
mod remove_duplicates_tests {
    use super::*;

    #[test]
    fn remove_test_one() {
        assert_eq!(2, remove_duplicates(&mut vec![1, 1, 2]))
    }
}
