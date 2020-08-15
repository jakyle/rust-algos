pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {

    let max = candies.iter().max().unwrap();

    let mut result = vec![false; candies.len()];

    for (i, candy) in candies.iter().enumerate()  {
        if extra_candies + candy >= *max {
            result[i] = true;
        }
    }

    result
}


#[cfg(test)]
mod find_numbers_with_even_digits_tests {
    use super::*;

    #[test]
    fn kids_with_candies_test_one() {

        // Input: candies = [2,3,5,1,3], extraCandies = 3
        // Output: [true,true,true,false,true] 


        // arrange
        let candies_test = vec![2,3,5,1,3];
        let extra_candies_test = 3;

        // act
        let result = kids_with_candies(candies_test, extra_candies_test);

        // assert
        assert_eq!(result, vec![true,true,true,false,true]);
    }

}