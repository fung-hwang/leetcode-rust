struct Solution;

impl Solution {
    // 基本思路：旋转数组，二分法
    // time: O(lgn)
    // space: O(1)
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return true;
            }
            if nums[mid] < nums[left] {
                // 右半段有序
                if target >= nums[mid] && target <= nums[right] {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            } else if nums[mid] > nums[left] {
                // 左半段有序
                if target >= nums[left] && target <= nums[mid] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else {
                // 用 nums[mid] 和 nums[left] 而非 nums[right] 比较的原因是：
                // 如果是 nums[right]，此处为 right -= 1 。因为 right 是 usize 类型，从0减为-1时会存在越界问题。
                // 但 left += 1 没有这个问题
                left += 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
        assert_eq!(Solution::search(vec![1, 1, 1], 1), true);
    }
}
