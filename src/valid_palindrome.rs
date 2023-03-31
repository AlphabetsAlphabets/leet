pub fn is_palindrome(s: String) -> bool {
    let s = s.to_lowercase();

    let original: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed: String = original.chars().rev().collect();

    original == reversed
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn strip_punctuation() {
        let original = String::from("A man, a plan, a canal: Panama");
        let res = is_palindrome(original);
        assert!(res);
    }

    #[test]
    fn race_a_car() {
        let original = String::from("race a car");
        let res = !is_palindrome(original);
        assert!(res);
    }

    #[test]
    fn empty_string() {
        let original = String::from(" ");
        let res = is_palindrome(original);
        assert!(res);
    }
}
