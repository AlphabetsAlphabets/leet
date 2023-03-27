pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut ans = 0;
        for i in 0..len {
            let mut cur_sum = 0;
            for j in i..len {
                cur_sum += nums[j];
                ans = std::cmp::max(ans, cur_sum);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn max_sub_array() {
        use maximum_subarray::Solution;
        Solution::max_sub_array(vec![1]);
    }
}
