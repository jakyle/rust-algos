pub fn repeated_n_times(A: Vec<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    let (mut i, mut j) = (0, 0);
    // approximately 0.5 * 0.5 probability per round
    // so average 4 turns to get the result
    while i == j || A[i] != A[j] {
        i = rand::Rng::gen_range(&mut rng, 0..A.len());
        j = rand::Rng::gen_range(&mut rng, 0..A.len());
    }
    A[i]
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
