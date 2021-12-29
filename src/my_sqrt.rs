pub fn my_sqrt(x: i32) -> i32 {
    let result = x >> 1;
    println!("{}", result);
    result
}

#[cfg(test)]
mod my_sqrt_tests {
    use super::*;

    #[test]
    fn my_sqrt_test_one() {
        assert_eq!(my_sqrt(8), 2);
        assert_eq!(my_sqrt(36), 6);
    }
}
