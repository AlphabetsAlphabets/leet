pub fn max_depth(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut max_depth = 0;
    let mut current_depth = 0;
    for c in s.chars() {
        if c == '(' {
            current_depth += 1;
            max_depth = std::cmp::max(max_depth, current_depth);
        } else if c == ')' {
            current_depth -= 1;
        }
    }

    max_depth
}

#[cfg(test)]
mod tests {
    use super::max_depth;

    #[test]
    fn ex1() {
        let s = String::from("(()(()))");
        let depth = max_depth(s);
        assert_eq!(depth, 3);
    }

    #[test]
    fn ex2() {
        let s = String::from("(1+(2*3)+((8)/4))+1");
        let depth = max_depth(s);
        assert_eq!(depth, 3);
    }

    #[test] 
    fn ex3() {
        let s = String::from("(1)+((2))+(((3)))");
        let depth = max_depth(s);
        assert_eq!(depth, 3);
    }
}
