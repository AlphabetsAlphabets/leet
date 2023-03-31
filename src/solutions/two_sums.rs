use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pos = vec![];
        let mut seen: HashMap<i32, usize> = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            let diff = target - num;
            if seen.contains_key(&diff) {
                let some_index = seen.get(&diff).unwrap();
                pos.push(*some_index as i32);
                pos.push(index as i32);

                break;
            } else {
                seen.insert(*num, index);
            }
        }

        pos
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn apart() {
        let nums = vec![2, 15, 11, 7];
        let target = 9;
        let res = Solution::two_sum(nums, target);

        assert_eq!(res, vec![0, 3]);
    }

    #[test]
    pub fn consecutive() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let res = Solution::two_sum(nums, target);

        assert_eq!(res, vec![0, 1]);
    }
}
