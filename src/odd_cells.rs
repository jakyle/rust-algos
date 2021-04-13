pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; m as usize]; n as usize];

    for point in indices {
        for x in matrix[point[0] as usize].iter_mut() {
            *x += 1;
        }

        for row in matrix.iter_mut() {
            *row.get_mut(point[1] as usize).unwrap() += 1;
        }
    }

    matrix.iter().flatten().filter(|&x| x % 2 != 0).count() as i32
}

#[cfg(test)]
mod odd_cells_tests {
    use super::*;

    #[test]
    fn odd_cells_tests_one() {
        // arrange
        let indices = vec![vec![0, 1], vec![1, 1]];
        let m = 2;
        let n = 3;

        // act
        let result = odd_cells(n, m, indices);

        // assert
        assert_eq!(result, 6);
    }
}
