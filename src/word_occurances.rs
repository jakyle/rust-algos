trait  Occurances<T> {
    fn occurances_of(&self, match_check: T) -> i32;
}

impl Occurances<&str> for String {

    fn occurances_of(&self, m: &str) -> i32 {
    
        use std::collections::HashMap;
    
        if &self.len() < &m.len() {return 0}
    
        let word_map = m.chars()
            .fold(HashMap::new(), |mut acc, ch| {
                *acc.entry(ch).or_insert(0) += 1;
                acc
            });
    
        let letter_counts = &self.chars()
            .filter(|ch| word_map.contains_key(&ch))
            .fold(HashMap::new(), |mut acc, ch| {
                acc.entry(ch)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
    
                acc
            });
    
        word_map
            .iter()
            .map(|(k, v)| {
                let res = letter_counts.get(&k).unwrap_or(&0) / v;
                res
            })
            .min()
            .unwrap()
    }
    
}





#[cfg(test)]
mod word_occurances_test {
    use super::*;

    #[test]
    fn match_just_word() {
        // arrange
        let test = String::from("letter");

        // act
        let result = test.occurances_of("letter");

        // assert
        assert_eq!(result, 1);
    }

    #[test]
    fn no_match_word() {
        // arrange
        let test = String::from("mail");

        // act
        let result = test.occurances_of("dog");

        // assert
        assert_eq!(result, 0);
    }

    #[test]
    fn no_match_just_uneven_word() {
        // arrange
        let test = String::from("letter");
        // act
        let result = test.occurances_of("letters");

        // assert
        assert_eq!(result, 0);
    }

    #[test]
    fn match_two_words() {
        // arrange
        let test = String::from("xxlxoeotootxxtiier;;ooilkjtjejt,,owwe..r");

        // act
        let result = test.occurances_of("letter");

        // assert
        assert_eq!(result, 2);
    }

    #[test]
    fn baloons_test() {
        let test = String::from("mbetypbpefxvviadqaodrbjeoacfomepmzymiudltgnvnpbowwmjgpzzhtiismearuwocsgbiimiqqzaozgeizikrlxmupfzjzmlfttqqbpfblqfkecsdfbsceqjhubfxksivrfwvukapxmuciybfhzlmpeamdxziptxregymqtmgcsujmugissgnlbhxbcxxeoumcqyulvahuianbaaxgzrtmshjguqdaxvxndzoqvwmcjfhpevavnrciqbymnlylbrfkkiceienoarfrzzxtuaqapaeqeqolozadmtgjyhfqzpuaskjuawxqkdqyjqcmbxtvshzrquvegcuyuckznspmrxvqdassidcmrajedsnuuumfwqzvasljlyvfefktiqgvzvdzojtjegsyhbepdkuwvgrfscezvswywmdavpxlekbrlkfnbyvlobazmvgulxrfdranuhomkrlpbfeagfxxxuhjuqhbkhznixquxrxngwimdxdhqbdaouitsvcdmbwxbbaomkgxsqwnexbjjyhtxvkjfqkrrxjghvzqsattubphryqxxdyjkihfnzvjhohnhdlfwoqiwtmwzfgcyhyqtcketvgnbchcxvnhcsoosirfqgdgcsitegzlxdfijzmxnvhrulmgvoqfpzesootscnxenokmmozmoxpaverydbsnimwacjqhrtxkqtvghjyushoctxphxzztukgmnoeycqaeukymvwxcsyvvctflqjhtcvjtxncuvhkptkjnzaetwbzkwnseovewuhpkaxiphdicgacszzdturzgjkzwgkmzzavykancvvzaafgzjhcyicorrblmhsnnkhfkujttbkuuedhwguuaapojmnjdfytdhrepjwcddzsoeutlbbljlikghxefgbqenwamanikmynjcupqpdjnhldaixwygcvsgdkzszmsptqqnroflgozblygtiyaxudwmooiviqcosjfksnevultrf");
        // act
        let result = test.occurances_of("balloon");

        // assert
        assert_eq!(result, 14);
    }

}