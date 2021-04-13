/// A string is a valid parentheses string (denoted VPS) if it meets one of the following:
///
///     It is an empty string "", or a single character not equal to "(" or ")",
///     It can be written as AB (A concatenated with B), where A and B are VPS's, or
///     It can be written as (A), where A is a VPS.
///
/// We can similarly define the nesting depth depth(S) of any VPS S as follows:
///
///     depth("") = 0
///     depth(C) = 0, where C is a string with a single character not equal to "(" or ")".
///     depth(A + B) = max(depth(A), depth(B)), where A and B are VPS's.
///     depth("(" + A + ")") = 1 + depth(A), where A is a VPS.
///
/// For example, "", "()()", and "()(()())" are VPS's (with nesting depths 0, 1, and 2), and ")(" and "(()" are not VPS's.
///
/// Given a VPS represented as string s, return the nesting depth of s.
///
///
///
/// Example 1:
///
/// Input: s = "(1+(2*3)+((8)/4))+1"
/// Output: 3
/// Explanation: Digit 8 is inside of 3 nested parentheses in the string.
///
/// Example 2:
///
/// Input: s = "(1)+((2))+(((3)))"
/// Output: 3
///
/// Example 3:
///
/// Input: s = "1+(2*3)/(2-1)"
/// Output: 1
///
/// Example 4:
///
/// Input: s = "1"
/// Output: 0
///
///
///
/// Constraints:
///
///     1 <= s.length <= 100
///     s consists of digits 0-9 and characters '+', '-', '*', '/', '(', and ')'.
///     It is guaranteed that parentheses expression s is a VPS.
pub fn max_depth(s: String) -> i32 {
    s.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .fold((0, 0), |(depth, max), change| {
            (depth + change, max.max(depth + change))
        })
        .1
}

#[cfg(test)]
mod max_depth_tests {
    use super::*;

    #[test]
    fn max_depth_tests_one() {
        // arrange
        let test = "(()(()))".to_string();

        // act
        let result = max_depth(test);

        // assert
        assert_eq!(result, 3);
    }

    #[test]
    fn max_depth_tests_two() {
        // arrange
        let test = "(1)+((2))+(((3)))".to_string();

        // act
        let result = max_depth(test);

        // assert
        assert_eq!(result, 3);
    }

    #[test]
    fn max_depth_tests_three() {
        // arrange
        let test = "1+(2*3)/(2-1)".to_string();

        // act
        let result = max_depth(test);

        // assert
        assert_eq!(result, 1);
    }
}
