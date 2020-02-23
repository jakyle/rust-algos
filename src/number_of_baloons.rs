pub fn max_number_of_balloons(s: String) -> i32 {
	let mut baloons = vec![0; 5];
	for ch in s.chars() {
		match ch {
			'b' => baloons[0] += 2,
			'a' => baloons[1] += 2,
			'l' => baloons[2] += 1,
			'o' => baloons[3] += 1,
			'n' => baloons[4] += 2,
			_ => (),
		}
    }
    
    *baloons.iter().min().unwrap() / 2
}


// balloon

#[cfg(test)]
mod baloons_tests {
    use super::*;

    #[test]
    fn has_two_baloons() {
        // arrange
        let test = String::from("balloonballoon");

        // act
        let result = max_number_of_balloons(test);

        // assert
        assert_eq!(result, 2);
    }

    #[test]
    fn has_three_baloons() {
        // arrange
        let test = String::from("balloonballoonballoon");

        // act
        let result = max_number_of_balloons(test);

        // assert
        assert_eq!(result, 3);
    }

    #[test]
    fn scrambled_baloons() {
        // arrange
        let test = String::from("loonbalxballpoon");

        // act
        let result: i32 = max_number_of_balloons(test);

        // assert
        assert_eq!(result, 2);
    }

    #[test]
    fn no_matches() {
        // arrange
        let test = String::from("leetcode");

        // act
        let result: i32 = max_number_of_balloons(test);

        // assert
        assert_eq!(result, 0);
    }

    #[test]
    fn matches_ten() {
        // arrange
        let test = String::from("krhizmmgmcrecekgyljqkldocicziihtgpqwbticmvuyznragqoyrukzopfmjhjjxemsxmrsxuqmnkrzhgvtgdgtykhcglurvppvcwhrhrjoislonvvglhdciilduvuiebmffaagxerjeewmtcwmhmtwlxtvlbocczlrppmpjbpnifqtlninyzjtmazxdbzwxthpvrfulvrspycqcghuopjirzoeuqhetnbrcdakilzmklxwudxxhwilasbjjhhfgghogqoofsufysmcqeilaivtmfziumjloewbkjvaahsaaggteppqyuoylgpbdwqubaalfwcqrjeycjbbpifjbpigjdnnswocusuprydgrtxuaojeriigwumlovafxnpibjopjfqzrwemoinmptxddgcszmfprdrichjeqcvikynzigleaajcysusqasqadjemgnyvmzmbcfrttrzonwafrnedglhpudovigwvpimttiketopkvqw");

        // act
        let result: i32 = max_number_of_balloons(test);

        // assert
        assert_eq!(result, 10);
    }
}