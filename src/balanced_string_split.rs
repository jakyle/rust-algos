/// 1221. Split a String in Balanced Strings
/// Easy
///
/// Balanced strings are those that have an equal quantity of 'L' and 'R' characters.
///
/// Given a balanced string s, split it in the maximum amount of balanced strings.
///
/// Return the maximum amount of split balanced strings.
///
///
///
/// Example 1:
///
/// Input: s = "RLRRLLRLRL"
/// Output: 4
/// Explanation: s can be split into "RL", "RRLL", "RL", "RL", each substring contains same number of 'L' and 'R'.
///
/// Example 2:
///
/// Input: s = "RLLLLRRRLR"
/// Output: 3
/// Explanation: s can be split into "RL", "LLLRRR", "LR", each substring contains same number of 'L' and 'R'.
///
/// Example 3:
///
/// Input: s = "LLLLRRRR"
/// Output: 1
/// Explanation: s can be split into "LLLLRRRR".
///
/// Example 4:
///
/// Input: s = "RLRRRLLRLL"
/// Output: 2
/// Explanation: s can be split into "RL", "RRRLLRLL", since each substring contains an equal number of 'L' and 'R'
///
///
///
/// Constraints:
///
///     1 <= s.length <= 1000
///     s[i] is either 'L' or 'R'.
///     s is a balanced string.

pub fn balanced_string_split(s: String) -> i32 {
    let mut acc = 0;
    let mut tracker = 0;

    for letter in s.chars() {
        tracker += if letter == 'L' { 1 } else { -1 };

        if tracker == 0 {
            acc += 1;
        }
    }

    acc
}

#[cfg(test)]
mod balanced_string_tests {
    use super::*;

    #[test]
    fn balanced_string_tests_one() {
        // arrange
        let test = "RLRRLLRLRL".to_string();

        // act
        let result = balanced_string_split(test);

        // assert
        assert_eq!(result, 4);
    }

    #[test]
    fn balanced_string_tests_two() {
        // arrange
        let test = "RLRRRLLRLL".to_string();

        // act
        let result = balanced_string_split(test);

        // assert
        assert_eq!(result, 2);
    }
}
