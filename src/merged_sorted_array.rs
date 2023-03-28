struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;

        // m + n the length of nums1 after elements inside
        // nums2 is inside it.
        let mut last = m + n - 1;

        while m > 0 && n > 0 {
            if nums1[m - 1] > nums2[n - 1] {
                nums1[last] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[last] = nums2[n - 1];
                n -= 1;
            }

            last -= 1;
        }

        while n > 0 {
            nums1[last] = nums2[n - 1];
            n -= 1;
            last -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn first_element_arranged() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];

        Solution::merge(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
