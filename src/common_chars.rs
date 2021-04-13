pub fn common_chars(a: Vec<String>) -> Vec<String> {
    use std::collections::HashMap;

    let mut char_maps: Vec<HashMap<char, i32>> = vec![];

    for word in a {
        let mut map = HashMap::new();

        for letter in word.chars() {
            *map.entry(letter).or_insert(0) += 1;
        }

        char_maps.push(map);
    }

    let test_map = char_maps.pop().unwrap_or(HashMap::new());

    let mut result: Vec<String> = vec![];

    for (letter, num) in test_map.iter() {
        let mut x = num.clone();

        for map in &char_maps {
            let val = *map.get(letter).unwrap_or(&0);

            if val < x {
                x = val;
            }
        }

        for _ in 0..x {
            result.push(letter.to_string())
        }
    }

    result
}

#[cfg(test)]
mod common_chars_tests {
    use super::*;

    #[test]
    fn common_chars_test1() {
        // arrange
        let input_one = vec![
            "bella".to_string(),
            "label".to_string(),
            "roller".to_string(),
        ];

        // act
        let result = common_chars(input_one);

        // assert
        assert_eq!(
            result,
            vec!["e".to_string(), "l".to_string(), "l".to_string()]
        );
    }
}
