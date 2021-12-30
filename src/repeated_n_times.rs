pub fn repeated_n_times(a: Vec<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    let (mut i, mut j) = (0, 0);
    // approximately 0.5 * 0.5 probability per round
    // so average 4 turns to get the result
    while i == j || a[i] != a[j] {
        i = rand::Rng::gen_range(&mut rng, 0..a.len());
        j = rand::Rng::gen_range(&mut rng, 0..a.len());
    }
    a[i]
}

#[cfg(test)]
mod repeated_n_times_tests {
    use super::*;

    #[test]
    fn repeated_n_times_test_one() {
        // arrange
        let test = vec![1, 2, 3, 3];

        // act
        let result = repeated_n_times(test);

        // assert
        assert_eq!(result, 3);
    }
}
