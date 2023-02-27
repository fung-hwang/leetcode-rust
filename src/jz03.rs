use crate::Solution;

impl Solution {
    // 基本思路是将数字交换到对应的下标处，即将 i 放到 nums[i] 处
    // “所有数字都在 0～n-1 的范围内”暗示了与数组下标的关系
    // time: O(n)
    // space: O(1)
    pub fn find_repeat_number(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            while i != nums[i] as usize {
                if nums[i] == nums[nums[i] as usize] {
                    return nums[i];
                } else {
                    let tmp = nums[i] as usize;
                    nums.swap(i, tmp);
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jz03() {
        assert_eq!(Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]), 2);
        assert_eq!(Solution::find_repeat_number(vec![3, 4, 2, 1, 1, 0]), 1);
    }
}
