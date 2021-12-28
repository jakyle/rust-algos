pub fn check_inclusion(window: String, subject: String) -> bool {
    let get_idx = |c: u8| (c - b'a') as usize;

    let mut window_alphabet_counter = [0u8; 26];
    window
        .bytes()
        .for_each(|b| window_alphabet_counter[get_idx(b)] += 1);

    let (window_len, subject_letters) = (window.len(), subject.as_bytes());
    let mut subject_alphabet_counter = [0u8; 26];
    for idx in 0..subject.len() {
        let current_letter = get_idx(subject_letters[idx]);
        subject_alphabet_counter[current_letter] += 1;

        if idx >= window_len {
            let letter_to_decrement = get_idx(subject_letters[idx - window_len]);
            subject_alphabet_counter[letter_to_decrement] -= 1;
        }

        if window_alphabet_counter == subject_alphabet_counter {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod check_inclusion_tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(check_inclusion("ab".into(), "eidbaooo".into()), true);
    }

    #[test]
    fn test_two() {
        assert_eq!(check_inclusion("abc".into(), "ccccbbbbaaaa".into()), false);
    }
}
