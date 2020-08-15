pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {

    let mut result = vec![0; nums.len()];

    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().enumerate() {
            
            if i == j {
                continue;
            }

            if y < x {
                result[i] += 1;
            }

        }
    }

    result
}

#[cfg(test)]
mod smaller_numbers_than_current_tests {
    use super::*;

    #[test]
    fn smaller_numbers_than_current_test_one() {

        // arrange
        let test = vec![8,1,2,2,3];

        // act
        let result = smaller_numbers_than_current(test);

        // assert
        assert_eq!(result, vec![4,0,1,1,3]);
    }

}