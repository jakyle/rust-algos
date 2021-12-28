const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];
const HUNDREDS: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
const TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

pub fn int_to_roman(num: i32) -> String {
    let n = num as usize;
    String::new()
        + THOUSANDS[n / 1000]
        + HUNDREDS[n % 1000 / 100]
        + TENS[n % 100 / 10]
        + ONES[n % 10]
}

#[cfg(test)]
mod int_to_roman_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn one_is_I() {
        assert_eq!(int_to_roman(1), "I".to_string());
    }

    #[test]
    fn nineteen_ninety_four() {
        assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
    }

    #[test]
    fn fifty_eight() {
        assert_eq!(int_to_roman(58), "LVIII".to_string());
    }

    #[test]
    fn ten() {
        assert_eq!(int_to_roman(10), "X".to_string());
    }

    #[test]
    fn three_hundred_and_six() {
        assert_eq!(int_to_roman(306), "CCCVI".to_string());
    }
}
