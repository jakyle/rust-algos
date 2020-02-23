
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut answer: Vec<i32> = vec![];

    use std::collections::HashMap;
    let mut seen: HashMap<i32, i32> = HashMap::new(); 

    for i in 0..nums.len() {
        let candidate = seen.get(&(target - nums[i]));

        if candidate.is_some() {
            answer.push(candidate.unwrap().clone());
            answer.push(i as i32);
        } else {
            seen.insert(nums[i], i as i32);
        }
    }

    answer
}


#[cfg(test)]
mod two_sum_tests {
    use super::*;

    #[test]
    fn nums_add_up(){
        // arrange
        let test = vec![5, 9, 3];
        let target = 8;

        // act
        let result = two_sum(test, target);

        // assert
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn nums_add_up_leet_example(){
        // arrange
        let test = vec![2, 7, 11, 15];
        let target = 9;

        // act
        let result = two_sum(test, target);

        // assert
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn nums_add_up_helix_example(){
        // arrange
        let test = vec![2, 7, 18, 15, 18];
        let target = 36;

        // act
        let result = two_sum(test, target);

        // assert
        assert_eq!(result, vec![2, 4]);
    }


}