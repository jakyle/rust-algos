pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    let u = |i: i32| i as usize;
    let i = |x: usize| x as i32;

    let start_color = image[u(sr)][u(sc)];

    if start_color == new_color {
        return image;
    }

    let (width, height) = (image.len(), image[0].len());
    let mut queue = std::collections::VecDeque::new();
    let dirs: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut visited = std::collections::HashSet::new();

    queue.push_front((u(sr), u(sc)));
    while let Some((row, col)) = queue.pop_back() {
        if visited.contains(&(row, col)) {
            continue;
        }

        image[row][col] = new_color;

        visited.insert((row, col));

        dirs.iter()
            .map(|(dx, dy)| (i(row) + dx, i(col) + dy))
            .filter(|&(x, y)| {
                x >= 0
                    && x < i(width)
                    && y >= 0
                    && y < i(height)
                    && image[u(x)][u(y)] == start_color
            })
            .for_each(|(x, y)| queue.push_front((u(x), u(y))));
    }

    image
}

#[cfg(test)]
mod flood_fill_tests {
    use super::*;

    #[test]
    fn flood_fill_test_one() {
        assert_eq!(
            flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 2),
            vec![vec![2, 2, 2], vec![2, 2, 2]]
        );
    }

    #[test]
    fn flood_fill_test_two() {
        assert_eq!(
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        )
    }
}
