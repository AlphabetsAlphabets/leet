use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    let mut matching_paren: HashMap<char, char> = HashMap::new();
    matching_paren.insert(')', '(');
    matching_paren.insert(']', '[');
    matching_paren.insert('}', '{');

    for c in s.chars() {
        if matching_paren.contains_key(&c) {
            let last = stack[stack.len() - 1];
            let paren = matching_paren.get(&c).unwrap();

            if !stack.is_empty() && last == *paren {
                stack.pop();
            } else {
                return false;
            }
        } else {
            stack.push(c);
        }
    }

    if stack.is_empty() {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test] 
    fn normal_bracket() {
        let res = is_valid("()".to_string());
        assert!(res);
    }

    #[test]
    fn square_bracket() {
        let res = is_valid("[]".to_string());
        assert!(res);
    }

    #[test]
    fn fancy_bracket() {
        let res = is_valid("{}".to_string());
        assert!(res);
    }

    #[test]
    fn everything() {
        let res = is_valid("{[([])]}".to_string());
        assert!(res);
    }

    #[test]
    fn mismatched() {
        let res = !is_valid("{]]".to_string());
        assert!(res);
    }

}
