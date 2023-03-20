struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let n = nums.len();
        let mut money = vec![0; n];
        if nums.len() == 1 {
            return nums[0];
        }
        money[0] = nums[0];
        money[1] = max(nums[0], nums[1]);
        // for i in 2..n {
        //     money[i] = std::cmp::max(money[i - 2] + nums[i], money[i - 1]);
        // }
        (2..n).for_each(|i| {
            money[i] = max(money[i - 2] + nums[i], money[i - 1]);
        });

        money[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![2, 7, 9, 100, 1]), 107);
        assert_eq!(Solution::rob(vec![2, 1]), 2);
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
    }
}
