pub fn find_numbers(x: Vec<i32>) -> i32 {
    let mut result = 0;

    for num in x {
        let mut y = num;
        let mut digits = 0;

        while y != 0 {
            y /= 10;
            digits += 1;
        }

        if digits % 2 == 0 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod find_numbers_with_even_digits_tests {
    use super::*;

    #[test]
    fn two_even_digits() {
        // arrange
        let test = vec![12, 345, 2, 6, 7896];

        // act
        let result = find_numbers(test);

        // assert
        assert_eq!(result, 2);
    }
}
