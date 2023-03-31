pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut cur_sum = 0;
        for value in nums {
            if cur_sum < 0 {
                cur_sum = 0;
            }

            cur_sum += value;
            max_sum = std::cmp::max(max_sum, cur_sum);
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn max_sub_array() {
        use maximum_subarray::Solution;
        let sol = Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(sol, 6, "SOL: {}", sol);
    }
}
