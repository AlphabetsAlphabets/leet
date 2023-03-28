pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index = vec![];
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let a = nums[i];
                let b = nums[j];

                if a + b == target {
                    index.push(i as i32);
                    index.push(j as i32);
                    break;
                }
            }
        }

        index
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
