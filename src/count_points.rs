fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![0; queries.len()];
    for (i, query) in queries.iter().enumerate() {
        let r_squared = i32::pow(query[2], 2);

        for point in &points {
            let distance_squared =
                i32::pow(query[0] - point[0], 2) + i32::pow(query[1] - point[1], 2);

            if distance_squared <= r_squared {
                result[i] += 1;
            }
        }
    }

    result
}
