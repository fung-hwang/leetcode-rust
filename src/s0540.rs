struct Solution;

impl Solution {
    // 在二分的过程中，我们需要注意的一个“恒量”是每次去除的一半元素应该是偶数个的。
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            if mid % 2 == 1 {
                if nums[mid] == nums[mid - 1] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            } else {
                if nums[mid] == nums[mid + 1] {
                    left = mid;
                } else {
                    right = mid;
                }
            }
        }
        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
            2
        );
        assert_eq!(
            Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
            10
        );
    }
}
