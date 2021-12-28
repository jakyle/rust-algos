pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let (mut start, mut end) = (0, nums.len() - 1);
    let mut idx = end;
    let mut results = vec![0; end + 1];
    while start < end {
        let start_num = nums[start].pow(2);
        let end_num = nums[end].pow(2);
        if start_num < end_num {
            results[idx] = end_num;
            end -= 1;
        } else {
            results[idx] = start_num;
            start += 1;
        }
        idx -= 1;
    }

    results[idx] = nums[start].pow(2);
    results
}

#[cfg(test)]
mod sorted_squares_tests {
    use super::*;

    #[test]
    fn first_sorted_squares_test() {
        assert_eq!(
            sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        )
    }
}
