pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let (width, height, mut total) = (grid.len(), grid[0].len(), 0);

    for x in 0..width {
        for y in 0..height {
            if grid[x][y] == 1 && !visited[x][y] {
                total = total.max(area_of_island(&grid, width, height, &mut visited, x, y));
            }
        }
    }
    total
}

fn area_of_island(
    grid: &Vec<Vec<i32>>,
    w: usize,
    h: usize,
    visited: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
) -> i32 {
    visited[x][y] = true;
    let vis = std::cell::RefCell::new(visited);
    let (mut stack, mut counter) = (vec![(x, y)], 0);
    static DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    while let Some((row, col)) = stack.pop() {
        counter += 1;
        DIRS.iter()
            .map(|(dx, dy)| ((row as i32 + dx) as usize, (col as i32 + dy) as usize))
            .filter(|&(x, y)| x < w && y < h && grid[x][y] == 1 && !vis.borrow()[x][y])
            .for_each(|(x, y)| {
                stack.push((x, y));
                vis.borrow_mut()[x][y] = true;
            });
    }
    counter
}

#[cfg(test)]
mod max_area_of_island_tests {
    use super::*;

    #[test]
    fn max_area_of_island_test_one() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(max_area_of_island(grid), 6);
    }

    #[test]
    fn max_area_of_island_test_two() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(max_area_of_island(grid), 1);
    }
}
