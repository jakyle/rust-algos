
pub fn reverse(x: i32) -> i32 {
        
    let mut x = x;
    let mut reverse_num = 0;

    while x != 0 {
        reverse_num = reverse_num * 10 + (x % 10);
        x /= 10;
    }

    reverse_num
}



#[cfg(test)]
mod reverse_integer_tests {
    use super::*;

    #[test]
    fn number_reverses(){
        // arrange
        let test = 123;

        // act
        let result = reverse(test);

        // assert
        assert_eq!(result, 321);
    }

    #[test]
    fn number_reverses_complex(){
        // arrange
        let test = 1122;

        // act
        let result = reverse(test);

        // assert
        assert_eq!(result, 2211);
    }

    #[test]
    fn negative_numbers() {
        // arrange
        let test = -123;

        // act
        let result = reverse(test);

        // assert
        assert_eq!(result, -321);
    }

}