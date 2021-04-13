pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let len = arr.len();

    let mut result = 0;

    for i in 0..len {
        for j in i + 1..len {
            for k in j + 1..len {
                if i32::abs(arr[i] - arr[j]) < a
                    && i32::abs(arr[j] - arr[k]) < b
                    && i32::abs(arr[i] - arr[k]) < c
                {
                    result += 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod count_good_triplets_test {
    use super::*;

    #[test]
    fn count_good_triplets_test_one() {
        // arrange
        let input_one = vec![3, 0, 1, 1, 9, 7];
        let input_two = 7;
        let input_three = 2;
        let input_four = 3;

        // act
        let result = count_good_triplets(input_one, input_two, input_three, input_four);

        // assert
        assert_eq!(result, 4);
    }
}
