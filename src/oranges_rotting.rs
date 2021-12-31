#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge(pub usize, pub usize, pub usize);

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.cmp(&self.2)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    let mut rotten_oranges = std::collections::BinaryHeap::new();
    grid.iter().enumerate().for_each(|(x, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, &orange)| orange == 2)
            .for_each(|(y, _)| rotten_oranges.push(Edge(x, y, 0)))
    });

    let mut total = 0;
    let (width, height) = (grid.len(), grid[0].len());
    let grid = std::cell::RefCell::new(grid);
    while let Some(Edge(r, c, d)) = rotten_oranges.pop() {
        total = total.max(d);
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .iter()
            .map(|(dx, dy)| ((dx + r as i32) as usize, (dy + c as i32) as usize))
            .filter(|&(x, y)| x < width && y < height && grid.borrow()[x][y] == 1)
            .for_each(|(x, y)| {
                rotten_oranges.push(Edge(x, y, d + 1));
                grid.borrow_mut()[x][y] = 2;
            })
    }

    if grid
        .into_inner()
        .into_iter()
        .flatten()
        .filter(|&x| x > 0)
        .all(|b| b == 2)
    {
        total as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod oranges_rotting_tests {
    use super::*;

    #[test]
    fn oranges_rotting_test_one() {
        assert_eq!(
            oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1],]),
            4
        );
    }

    #[test]
    fn oranges_rotting_test_two() {
        assert_eq!(
            oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 1], vec![0, 1, 2],]),
            2
        );
    }

    #[test]
    fn oranges_rotting_test_three() {
        assert_eq!(
            oranges_rotting(vec![vec![2, 2], vec![1, 1], vec![0, 0], vec![2, 0],]),
            1
        );
    }
}
