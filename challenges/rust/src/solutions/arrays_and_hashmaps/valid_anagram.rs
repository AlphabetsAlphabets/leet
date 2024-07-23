use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut seen: HashMap<char, usize> = HashMap::new();

    let s_iter = s.chars();
    let t_iter = t.chars();
    let s_and_t = s_iter.zip(t_iter);

    for (char_s, char_t) in s_and_t {
        seen.entry(char_s)
            .and_modify(|occurance| *occurance += 1)
            .or_insert(1);

        seen.entry(char_t)
            .and_modify(|occurance| *occurance += 1)
            .or_insert(1);
    }

    seen.values().find(|occurance| *occurance % 2 != 0).is_none()
}

#[cfg(test)]
mod tests {
    use crate::arrays_and_hashmaps::valid_anagram::is_anagram;

    #[test]
    fn differing_lengths() {
        let res = is_anagram("ababa".to_string(), "c".to_string());
        assert_eq!(res, false);
    }

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
