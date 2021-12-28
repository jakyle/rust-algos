pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut i = 0;
    for j in 0..nums.len() {
        if nums[j] != 0 {
            nums.swap(i, j);
            i += 1;
        }
    }
}

#[cfg(test)]
mod move_zeroes_tests {
    use super::*;

    #[test]
    fn move_zeroes_one() {
        let mut nums = vec![0, 1, 0, 3, 12];

        move_zeroes(&mut nums);

        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn move_zeroes_two() {
        let mut nums = vec![0];

        move_zeroes(&mut nums);

        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn move_zeroes_three() {
        let mut nums = vec![0, 0, 1];

        move_zeroes(&mut nums);

        assert_eq!(nums, vec![1, 0, 0]);
    }

    #[test]
    fn move_zeroes_four() {
        let mut nums = vec![2, 9, 0, 6, 1];

        move_zeroes(&mut nums);

        assert_eq!(nums, vec![2, 9, 0, 6, 1]);
    }
}
