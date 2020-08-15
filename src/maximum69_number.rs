pub fn maximum69_number (num: i32) -> i32 {
    let mut num = num;

    let mut numbers: Vec<i32> = vec![];

    while num > 0 {
        numbers.push(num % 10);
        num /= 10;
    }

    numbers.reverse();
    let total_position: (usize, i32) = (0, 0);
    for (i, x) in numbers.iter().enumerate() {
       let didFlipNumber = false;

       
    }

    2
}


#[cfg(test)]
mod maximum69_number_tests {
    use super::*;

    #[test]
    fn maximum69_number_test_one() {

        // arrange
        let test = 9669;

        // act
        let result = maximum69_number(test);

        // assert
        assert_eq!(result, 9969);
    }

}