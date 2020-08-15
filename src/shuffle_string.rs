pub fn restore_string(s: String, indices: Vec<i32>) -> String {

    let mut combo: Vec<(&i32, char)> = indices
        .iter()
        .zip(s.chars())
        .collect();
    
    combo.sort_by(|a, b| a.0.cmp(b.0));

    combo.iter()
        .map(|x| x.1)
        .collect()
}


#[cfg(test)]
mod shuffle_string_tests {
    use super::*;

    #[test]
    fn min_time_to_visit_all_points_test_one() {

        // arrange
        let test = vec![4,5,6,7,0,2,1,3];
        let test2 = String::from("codeleet");

        // act
        let result = restore_string(test2, test);

        // assert
        assert_eq!(result, String::from("leetcode"));
    }


}