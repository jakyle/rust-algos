pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let u = |i: i32| i as usize;
    let i = |x: usize| x as i32;

    let (width, height) = (grid.len(), grid[0].len());
    let mut queue = std::collections::VecDeque::new();
    let dirs: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut visited = std::collections::HashSet::new();
    let (mut counter, mut total) = (0, 0);
    queue.push_front((0, 0, grid[0][0]));
    while let Some((x, y, val)) = queue.pop_back() {
        if visited.contains(&(x, y, val)) {
            continue;
        }

        counter = if val == 1 { counter + 1 } else { 0 };
        total = total.max(counter);

        visited.insert((x, y, val));

        let neighbors = dirs.iter()
            .map(|(dx, dy)| (i(x) + dx, i(y) + dy))
            .filter(|&(x, y)| {
                x >= 0
                    && x < i(width)
                    && y >= 0
                    && y < i(height)
            });

        for (x, y) in neighbors {
            let cell = (u(x), u(y), grid[u(x)][u(y)]);

            if cell.2 == 1 {
                queue.push_back(cell);
            } else {
                queue.push_front(cell);
            } 
        }
    }

    total
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