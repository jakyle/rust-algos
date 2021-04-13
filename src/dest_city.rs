pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    use std::collections::HashSet;
    let src_cities: HashSet<&String> = paths.iter().map(|v| &v[0]).collect();
    paths
        .iter()
        .filter(|v| !src_cities.contains(&v[1]))
        .next()
        .unwrap()[1]
        .clone()
}

#[cfg(test)]
mod dest_city_tests {
    use super::*;

    #[test]
    fn dest_city_test_one() {
        // arrange
        let test = vec![
            vec![String::from("London"), String::from("New York")],
            vec![String::from("New York"), String::from("Lima")],
            vec![String::from("Lima"), String::from("Sao Paulo")],
        ];

        // act
        let result = dest_city(test);

        // assert
        assert_eq!(result, String::from("Sao Paulo"));
    }
}
