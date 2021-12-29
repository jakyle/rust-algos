pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let (width, height, mut total) = (grid.len(), grid[0].len(), 0);

    for x in 0..width {
        for y in 0..height {
            if grid[x][y] == 1 && !visited[x][y] {
                total = total.max(area_of_island(&grid, &mut visited, x, y));
            }
        }
    }

    total
}

fn area_of_island(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> i32 {
    let dirs: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (width, height) = (grid.len(), grid[0].len());
    let mut counter = 0;
    visited[x][y] = true;
    let mut stack = vec![(x, y)];
    while let Some((r, c)) = stack.pop() {
        counter += 1;
        let island_neighbors = dirs.iter()
            .map(|(dx, dy)| (r as i32 + dx, c as i32 + dy))
            .filter(|&(x, y)| {
                x >= 0
                    && x < width as i32
                    && y >= 0
                    && y < height as i32
                    && grid[x as usize][y as usize] == 1
            })
            .map(|(x, y)| (x as usize, y as usize));

        for (x, y) in island_neighbors {
            if !visited[x][y] {
                stack.push((x, y));
                visited[x][y] = true;
            }
        }
    }

    counter
}

#[cfg(test)]
mod max_area_of_island_tests {
    use super::*;

    #[test]
    fn max_area_of_island_test_one() {
        let grid = vec![
            vec![0,0,1,0,0,0,0,1,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
            vec![0,1,1,0,1,0,0,0,0,0,0,0,0],
            vec![0,1,0,0,1,1,0,0,1,0,1,0,0],
            vec![0,1,0,0,1,1,0,0,1,1,1,0,0],
            vec![0,0,0,0,0,0,0,0,0,0,1,0,0],
            vec![0,0,0,0,0,0,0,1,1,1,0,0,0],
            vec![0,0,0,0,0,0,0,1,1,0,0,0,0]
        ];
        assert_eq!(max_area_of_island(grid), 6);
    }

    #[test]
    fn max_area_of_island_test_two() {
        let grid = vec![
            vec![0,1],
            vec![1,0]
        ];
        assert_eq!(max_area_of_island(grid), 1);
    }
}