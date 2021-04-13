pub fn count_primes(n: i32) -> i32 {
    let mut acc = 0;
    if n <= 1 {
        return acc;
    }

    let mut n = n;

    while n > 1 {
        if n % 2 != 0 {
            acc += 1;
        }

        n -= 1;
    }

    acc
}

#[cfg(test)]
mod count_primes_tests {
    use super::*;

    #[test]
    fn count_primes_tests_one() {
        // arrange
        let test = 10;

        // act
        let result = count_primes(test);

        // assert
        assert_eq!(result, 4);
    }
}
