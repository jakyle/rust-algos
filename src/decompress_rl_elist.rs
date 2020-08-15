pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {

    let mut output: Vec<i32> = vec![];


    for x in nums.chunks(2) {

        let freq = x[0] as usize;
        let val = x[1];

        let mut insert_vec = vec![val; freq];
        
        output.append(&mut insert_vec);
    }

    output
}

#[cfg(test)]
mod decompress_rl_elist_tests {
    use super::*;

    #[test]
    fn decompress_rl_elist_test_one() {

        // arrange
        let test = vec![1,2,3,4];

        // act
        let result = decompress_rl_elist(test);

        // assert
        assert_eq!(result, vec![2,4,4,4]);
    }

    #[test]
    fn decompress_rl_elist_test_two() {

        // arrange
        let test = vec![1,1,2,3];

        // act
        let result = decompress_rl_elist(test);

        // assert
        assert_eq!(result, vec![1,3,3]);
    }
}