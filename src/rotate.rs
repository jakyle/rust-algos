pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = (k as usize) % nums.len();
    nums.rotate_right(k)
}

#[cfg(test)]
pub mod rotate_tests {
    use super::*;

    #[test]
    pub fn rotate_test_one() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);

        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    pub fn rotate_test_two() {
        let mut nums = vec![1, 2];
        rotate(&mut nums, 3);

        assert_eq!(nums, vec![2, 1]);
    }
}
