pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut n = n;

    let mut product = 1;
    let mut sum = 0;

    while n > 0 {
        let num = n % 10;

        product *= num;
        sum += num;

        n /= 10;
    }

    product - sum
}

#[cfg(test)]
mod subtract_product_and_sum_tests {
    use super::*;

    #[test]
    fn subtract_product_and_sum_test_one() {
        // arrange
        let test = 234;

        // act
        let result = subtract_product_and_sum(test);

        // assert
        assert_eq!(result, 15);
    }

    #[test]
    fn subtract_product_and_sum_test_two() {
        // arrange
        let test = 4421;

        // act
        let result = subtract_product_and_sum(test);

        // assert
        assert_eq!(result, 21);
    }
}
