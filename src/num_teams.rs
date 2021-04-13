pub fn num_teams(rating: Vec<i32>) -> i32 {
    let mut teams = 0;
    let len = rating.len();

    for i in 0..len {
        for j in i..len {
            for k in j..len {
                if (rating[i] < rating[j] && rating[j] < rating[k])
                    || (rating[i] > rating[j] && rating[j] > rating[k])
                {
                    teams += 1;
                }
            }
        }
    }

    teams
}

#[cfg(test)]
mod num_teams_tests {
    use super::*;

    #[test]
    fn num_teams_tests_one() {
        // arrange
        let test = vec![2, 5, 3, 4, 1];

        // act
        let result = num_teams(test);

        // assert
        assert_eq!(result, 3);
    }

    #[test]
    fn num_teams_tests_two() {
        // arrange
        let test = vec![2, 1, 3];

        // act
        let result = num_teams(test);

        // assert
        assert_eq!(result, 0);
    }

    #[test]
    fn num_teams_tests_three() {
        // arrange
        let test = vec![1, 2, 3, 4];

        // act
        let result = num_teams(test);

        // assert
        assert_eq!(result, 4);
    }
}
