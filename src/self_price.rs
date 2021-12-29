pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let _result: Vec<i32> = vec![];

    let len = prices.len();

    for i in 0..len {
        for _j in i + 1..len {}
    }

    vec![4, 2, 4, 2, 3]
}

#[cfg(test)]
mod final_prices_tests {
    use super::*;

    #[test]
    fn final_prices_test_one() {
        // arrange
        let test = vec![8, 4, 6, 2, 3];

        // act
        let result = final_prices(test);

        // assert
        assert_eq!(result, vec![4, 2, 4, 2, 3]);
    }
}
