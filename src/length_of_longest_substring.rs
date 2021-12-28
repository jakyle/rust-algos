pub fn length_of_longest_substring(s: &str) -> i32 {
    let mut map = [-1; 128];
    let (mut left, mut length) = (0, 0);
    for (right, letter) in s.bytes().enumerate() {
        left = left.max(map[letter as usize]+1);
        length = length.max(right as i16 - left+1);
        map[letter as usize] = right as i16;
    }
    
    length as i32
}

#[cfg(test)]
mod length_of_longest_substring_tests {
    use super::*;


    #[test]
    fn test_one() {
        assert_eq!(length_of_longest_substring("pwwkew"), 3);
    }

    #[test]
    fn test_two() {
        assert_eq!(length_of_longest_substring("abcdeafbdgcbb"), 7);
    }
}