use core::panic;

struct Solution;

impl Solution {
    // DP: temp_max_sum[i] = max(nums[i], nums[i] + temp_max_sum[i-1])
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            panic!();
        }
        let mut max_sub_vec_sum = nums[0];
        let mut temp_max_sum = nums[0];
        for i in 1..nums.len() {
            temp_max_sum = std::cmp::max(nums[i], nums[i] + temp_max_sum); // 到i为止的连续子数组的最大和
            max_sub_vec_sum = std::cmp::max(temp_max_sum, max_sub_vec_sum);
        }
        max_sub_vec_sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![-2]), -2);
    }
}
