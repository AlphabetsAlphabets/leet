use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hm: HashMap<i32, u8> = HashMap::new();
    for num in nums {
        if hm.contains_key(&num) {
            return true;
        } else {
            hm.insert(num, 1);
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::contains_duplicate;

    #[test]
    fn case1() {
        let test1 = contains_duplicate(vec![1, 2, 3, 1]);
        assert!(test1);

    }

    #[test]
    fn case2() {
        let test2 = contains_duplicate(vec![1, 2, 3, 4]);
        assert_eq!(test2, false);
    }

    #[test]
    fn case3() {
        let test3 = contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
        assert!(test3);
    }
}
