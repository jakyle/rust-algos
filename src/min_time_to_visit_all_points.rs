pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    pub fn find_distance_to(&self, point: &Point) -> i32 {
        use std::cmp;

        let distance = cmp::max(
            i32::abs(self.x - point.x), 
            i32::abs(self.y - point.y)
        );

        distance
    }
}


pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    
    let coordinates: Vec<Point> = points.iter()
        .map(|point| Point::new(point[0], point[1]))
        .collect();

    let mut total_time: i32 = 0;
    for i in 0..coordinates.len() -1 {
        total_time += coordinates[i].find_distance_to(&coordinates[i+1]);
    }
    
    total_time
}



#[cfg(test)]
mod min_time_to_visit_all_points_tests {
    use super::*;

    #[test]
    fn min_time_to_visit_all_points_test_one() {

        // arrange
        let test = vec![vec![1,1], vec![3,4], vec![-1,0]];

        // act
        let result = min_time_to_visit_all_points(test);

        // assert
        assert_eq!(result, 7);
    }

    #[test]
    fn min_time_to_visit_all_points_test_two() {

        // arrange
        let test = vec![vec![3, 2], vec![-2, 2]];

        // act
        let result = min_time_to_visit_all_points(test);

        // assert
        assert_eq!(result, 5);
    }

}