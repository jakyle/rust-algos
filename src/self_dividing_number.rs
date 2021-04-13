pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    (left..=right)
        .filter(is_self_dividing)
        .collect::<Vec<i32>>()
}

pub fn is_self_dividing(num: &i32) -> bool {
    let mut num_copy = *num;
    while num_copy != 0 {
        let x = num_copy % 10;

        if x == 0 {
            return false;
        }

        if !(num % x == 0) {
            return false;
        }

        num_copy /= 10;
    }

    true
}

#[cfg(test)]
mod self_dividing_numbers_tests {
    use super::*;

    #[test]
    fn self_dividing_numbers_one() {
        // arrange
        let lower = 1;
        let upper = 22;

        // act
        let result = self_dividing_numbers(lower, upper);

        // assert
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]);
    }
}
