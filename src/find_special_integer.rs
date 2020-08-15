pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;


    let mut kvp_arr: Vec<_> = arr.iter()
        .fold(HashMap::new(), |mut acc, val| {
            let entry =  acc.entry(*val).or_insert(0);
            *entry += 1;
            acc
        })
        .into_iter()
        .collect();
    
    kvp_arr.sort_by(|x,y| y.1.cmp(&x.1));

    kvp_arr[0].0
}

#[cfg(test)]
mod find_special_integer_tests {
    use super::*;

    #[test]
    fn find_special_integer_one() {

        // arrange
        let test = vec![1,2,2,6,6,6,6,7,10];

        // act
        let result = find_special_integer(test);

        // assert
        assert_eq!(result, 6);
    }

}