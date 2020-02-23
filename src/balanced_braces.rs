pub fn is_braces_balanced(braces: &str) -> bool {

    if braces.len() % 2 != 0 {
        return false;
    }

    let mut brace_stack: Vec<char> = vec![];

    for brace in braces.chars() {
        match brace {
            '(' | '<' | '{' | '[' => brace_stack.push(brace),
            closed_brace => {
                match brace_stack.pop() {
                    Some(open_brace) => {
                        match (open_brace, closed_brace) {
                              ('(', ')') 
                            | ('<', '>') 
                            | ('{', '}') 
                            | ('[', ']') => continue,
                            _ => return false
                        };
                    }   
                    None => return false
                }
            }
        }
    }

    brace_stack.is_empty()
}


#[cfg(test)]
mod stack_tests {
    use super::*;

    #[test]
    fn simple_braces_are_balanced() {
        // arrange
        let test = "(([]))";
        // act
        let result = is_braces_balanced(test);
        // assert
        assert!(result, true);
    }

    #[test]
    fn braces_len_odd_not_balanced() {
        // arrange
        let test = "(({[]))";
        // act
        let result = is_braces_balanced(test);
        // assert
        assert_eq!(result, false);
    }

    #[test]
    fn simple_braces_not_balanced() {
        // arrange
        let test = "<<{)>]";
        // act
        let result = is_braces_balanced(test);
        // assert
        assert_eq!(result, false);
    }


    #[test]
    fn nested_braces_not_balanced() {
        // arrange
        let test = "(()(<()[{}{}[]]>))";

        // act
        let result = is_braces_balanced(test);

        // assert
        assert_eq!(result, true);
    }
}