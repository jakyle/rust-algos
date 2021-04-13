pub fn duplicate_zeroes(arr: &mut Vec<i32>) {
    let mut i: i32 = 0;

    let len: i32 = arr.len() as i32;

    while i < len {
        if arr[i as usize] == 0 {
            arr.remove((len - 1) as usize);
            arr.insert(i as usize, 0);

            i += 2;
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod duplicate_zeroes_test {
    use super::*;

    #[test]
    fn duplicate_zeroes_test_onen() {
        // arrange
        let mut test = vec![1, 0, 2, 3, 0, 4, 5, 0];

        // act
        duplicate_zeroes(&mut test);

        // assert
        assert_eq!(test, vec![1, 0, 0, 2, 3, 0, 0, 4]);
    }
}
