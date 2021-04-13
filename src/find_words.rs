macro_rules! hashset {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashset!(@single $rest)),*]));

    ($($key:expr,)+) => { hashset!($($key),+) };
    ($($key:expr),*) => {
        {
            let _cap = hashset!(@count $($key),*);
            let mut _set = std::collections::HashSet::with_capacity(_cap);
            $(
                let _ = _set.insert($key);
            )*
            _set
        }
    };
}

pub fn find_words(words: Vec<String>) -> Vec<String> {
    let rows = vec![
        hashset! {'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'},
        hashset! {'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'},
        hashset! {'z', 'x', 'c', 'v', 'b', 'n', 'm'},
    ];

    words
        .into_iter()
        .filter(|word| {
            rows.iter()
                .any(|set| word.to_ascii_lowercase().chars().all(|c| set.contains(&c)))
        })
        .collect()
}

#[cfg(test)]
mod find_words_tests {
    use super::*;

    #[test]
    fn find_words_test_one() {
        // arrange
        let test = vec![
            String::from("Hello"),
            String::from("Alaska"),
            String::from("Dad"),
            String::from("Peace"),
        ];

        // act
        let result = find_words(test);

        // assert
        assert_eq!(result, vec!["Alaska", "Dad"]);
    }
}
