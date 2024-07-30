pub fn longest_common_prefix(strings: Vec<String>) -> String {
    let mut prefix = String::new();
    for string in strings {
        for index in 1..string.chars().count() {
            let contending_prefix = (&string[0..index]).to_string();
            prefix.push_str(&contending_prefix);

            if !prefix.starts_with(&contending_prefix) {
                break;
            }

        }
    }

    prefix
}

#[cfg(test)]
mod tests {
    use crate::longest_common_prefix::longest_common_prefix;

    #[test]
    fn fl_prefix() {
        let strings = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];

        let longest_prefix = longest_common_prefix(strings);
        assert_eq!(longest_prefix, "fl".to_string());
    }

    #[test]
    fn a_prefix() {
        let strings = vec!["ab".to_string(), "a".to_string()];

        let longest_prefix = longest_common_prefix(strings);
        assert_eq!(longest_prefix, "a".to_string());
    }

    #[test]
    fn f_prefix() {
        let strings = vec!["fwer".to_string(), "fow".to_string(), "flight".to_string()];

        let longest_prefix = longest_common_prefix(strings);
        assert_eq!(longest_prefix, "f".to_string());
    }
}
