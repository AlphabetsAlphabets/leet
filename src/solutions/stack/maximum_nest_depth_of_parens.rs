pub fn max_depth(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut depth = 0;
    let stack: Vec<char> = s.chars().filter(|c| *c == '(' || *c == ')').collect();

    for c in stack {
        if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
        }
    }

    depth
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
