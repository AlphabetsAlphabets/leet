use std::collections::HashMap;

pub struct Solution;

impl Solution {
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
}

#[cfg(test)]
mod tests {
    use crate::arrays_and_hashmaps::contains_duplicate;

    #[test]
    fn contains_duplicate() {
        use contains_duplicate::Solution;

        let test1 = Solution::contains_duplicate(vec![1, 2, 3, 1]);
        assert!(test1);

        let test2 = Solution::contains_duplicate(vec![1, 2, 3, 4]);
        assert_eq!(test2, false);

        let test3 = Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
        assert!(test3);
    }

}
