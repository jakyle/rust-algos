pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter().map(|a| a.iter().sum()).max().unwrap()
}

#[cfg(test)]
mod maximum_wealth_tests {
    use super::*;

    #[test]
    fn maximum_wealth_test1() {
        // arrange
        let input_one = vec![vec![1, 2, 3], vec![3, 2, 1]];

        // act
        let result = maximum_wealth(input_one);

        // assert
        assert_eq!(result, 6);
    }

    #[test]
    fn maximum_wealth_test2() {
        // arrange
        let input_one = vec![vec![1, 5], vec![3, 7], vec![3, 5]];

        // act
        let result = maximum_wealth(input_one);

        // assert
        assert_eq!(result, 10);
    }
}
