pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    points
        .iter()
        .enumerate()
        .filter(|(_, point)| x == point[0] || y == point[1])
        .map(|(i, point)| (i, i32::abs(x - point[0]) + i32::abs(y - point[1])))
        .fold((i32::MAX, -1), |(dist, idx), (i, x)| {
            if x < dist {
                (x, i as i32)
            } else {
                (dist, idx)
            }
        })
        .1
}

#[cfg(test)]
mod nearest_valid_point_tests {
    use super::*;

    #[test]
    fn nearest_valid_point_tests_one() {
        // arrange
        let points = vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]];
        let x = 3;
        let y = 4;

        // act
        let result = nearest_valid_point(x, y, points);

        // assert
        assert_eq!(result, 2);
    }
}
