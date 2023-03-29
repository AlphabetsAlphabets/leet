use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut chars_in_s: HashMap<char, usize> = HashMap::new();
    for char in s.chars() {
        chars_in_s
            .entry(char)
            .and_modify(|occurance| *occurance += 1)
            .or_insert(1);
    }

    let mut chars_in_t: HashMap<char, usize> = HashMap::new();
    for char in t.chars() {
        chars_in_t
            .entry(char)
            .and_modify(|occurance| *occurance += 1)
            .or_insert(1);
    }

    chars_in_s == chars_in_t
}


#[cfg(test)]
mod tests {
    use crate::valid_anagram::*;

    #[test]
    fn anagram() {
        let res = is_anagram("anagram".to_string(), "nagaram".to_string());
        assert!(res);
    }

    #[test]
    fn rat() {
        let res = is_anagram("rat".to_string(), "car".to_string());
        assert_eq!(res, false);
    }
}
