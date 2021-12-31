pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (width, height) = (mat.len(), mat[0].len());
    let distance = std::cell::RefCell::new(vec![vec![i32::MAX; height]; width]);
    let mut stack = vec![];
    for x in 0..width {
        for y in 0..height {
            if mat[x][y] == 0 {
                distance.borrow_mut()[x][y] = 0;
                stack.push((x, y))
            }
        }
    }

    while let Some((row, col)) = stack.pop() {
        const DIRS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        DIRS.iter()
            .map(|(dx, dy)| ((dx + row as i32) as usize, (dy + col as i32) as usize))
            .filter(|&(dx, dy)| {
                dx < width
                    && dy < height
                    && distance.borrow()[dx][dy] > distance.borrow()[row][col] + 1
            })
            .for_each(|(dx, dy)| {
                let mut dist = distance.borrow_mut();
                dist[dx][dy] = dist[row][col] + 1;
                stack.push((dx, dy));
            })
    }

    distance.into_inner()
}

#[cfg(test)]
mod update_matrix_tests {
    use super::*;

    #[test]
    fn update_matrix_test_one() {
        assert_eq!(
            update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
    }
}
