pub fn fib(n: i32) -> i32 {
    const SQRT5: f64 = 2.23606797749979;
    const PHI: f64 = (1.0 + SQRT5) / 2.0;
    const PSI: f64 = 1.0 - PHI;

    ((PHI.powf(n as f64) - PSI.powf(n as f64)) / SQRT5) as i32
}

#[cfg(test)]
mod fib_tests {
    use super::*;

    #[test]
    fn fib_test_one() {
        // arrange
        let test = 2;

        // act
        let result = fib(test);

        // assert
        assert_eq!(result, 1);
    }

    #[test]
    fn fib_test_two() {
        // arrange
        let test = 3;

        // act
        let result = fib(test);

        // assert
        assert_eq!(result, 2);
    }
}
