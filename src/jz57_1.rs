struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, nums.len() - 1);
        while i < j {
            let sum = nums[i] + nums[j];
            if sum == target {
                return vec![nums[i], nums[j]];
            } else if sum < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jz57_1() {
        assert_eq!(
            Solution::two_sum(vec![10, 26, 30, 31, 47, 60], 40),
            vec![10, 30]
        );
    }
}
