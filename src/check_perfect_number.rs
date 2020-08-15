pub fn check_perfect_number(num: i32) -> bool {
    
    let result: f64 = (2..num + 1)
        .map(|x| num as f64 / x as f64)
        .filter(|x| x % 1.0 == 0.0)
        .sum();

    result as i32 == num

    // for x in 1..num {
    //     let x = x as f64;

    //     let y = num_divisor / x;

    //     if y % 1.0  == 0.0 {
    //         result += x;
    //     }

    // }

    // num_divisor == result
}

#[cfg(test)]
mod check_perfect_number_tests {
    use super::*;

    #[test]
    fn check_perfect_number_test_one() {

        // arrange
        let test = 28;

        // act
        let result = check_perfect_number(test);

        // assert
        assert_eq!(result, true);
    }
}