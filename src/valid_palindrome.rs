pub fn is_palindrome(s: String) -> bool {
    let s = s.to_lowercase();
    let stripped: String = s.chars().filter(|c| c.is_alphanumeric()).collect();

    let mut start = 0;
    let mut end = stripped.len();
    let chars: Vec<char> = stripped.chars().collect();
    while start < end && end != 0 {
        let front = chars[start];
        let back = chars[end - 1];

        if front != back {
            return false;
        }

        start += 1;
        end -= 1;
    }

    true
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
