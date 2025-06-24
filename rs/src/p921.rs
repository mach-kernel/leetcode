/*
 "()))((" || ))((
 ()
 )))


")())("

 )()

 */

pub fn min_add_to_make_valid(s: String) -> i32 {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        if let Some(top) = stack.first() {
            if (*top == '(' && c == ')') {
                stack.remove(0);
                continue;
            }
        }

        stack.insert(0, c);
    }

    stack.len() as i32
}

#[cfg(test)]
mod test {
    use crate::p921::min_add_to_make_valid;

    #[test]
    fn test_min_add_to_make_valid() {
        assert_eq!(min_add_to_make_valid("())".to_string()), 1);
        assert_eq!(min_add_to_make_valid("(((".to_string()), 3);
        assert_eq!(min_add_to_make_valid("()))((".to_string()), 4);
        assert_eq!(min_add_to_make_valid(")()".to_string()), 1);
        assert_eq!(min_add_to_make_valid(")())(".to_string()), 3);
    }
}
