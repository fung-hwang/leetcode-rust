struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 || n == 2 {
            return 0;
        }
        let mut cnt = 0;
        let mut tmp_cnt = 0;
        for i in 2..n {
            if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
                tmp_cnt += 1; // tmp_cnt 延续 + 1 new
                cnt += tmp_cnt;
            } else {
                tmp_cnt = 0;
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![1, 2, 3, 5, 6, 7]),
            2
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4, 5]),
            6
        );
    }
}
