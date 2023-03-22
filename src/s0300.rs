struct Solution;

impl Solution {
    // time: O(n^2)
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let n = nums.len();
        if n == 0 || n == 1 {
            return n as i32;
        }
        let mut max_len = 1;
        let mut len = vec![1; n];
        for i in 0..n {
            for j in (0..i).rev() {
                if nums[j] < nums[i] {
                    len[i] = max(len[i], len[j] + 1);
                    max_len = max(max_len, len[i]);
                }
            }
        }

        max_len
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![1, 1, 1, 1, 1, 1]), 1);
    }
}
