pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        
    let mut nums = nums;

    nums.sort();

    let length = nums.len();

    if length < k as usize {
        nums[0]
    } else {
        nums[(length - k as usize)]
    }
}


#[cfg(test)]
mod find_kth_largest_test {
    use super::*;

    #[test]
    fn find_second_largest() {
        // arrange
        let test = vec![3, 2, 1, 5, 6, 4];
        let k = 2;

        // act
        let result = find_kth_largest(test, k);

        // assert
        assert_eq!(result, 5);
    }

    #[test]
    fn find_fourth_largest() {
        // arrange
        let test = vec![3,2,3,1,2,4,5,5,6];
        let k = 4;

        // act
        let result = find_kth_largest(test, k);

        // assert
        assert_eq!(result, 4);
    }

}