pub fn longest_common_prefix(strings: Vec<String>) -> String {
    let mut prefix = strings[0].clone();
    for string in strings {
        for (index, char) in string.chars().enumerate() {
            if prefix.contains(char) {
                continue;
            }

            prefix = (&string[0..index]).to_string();
            break;
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
