struct Solution;

impl Solution {
    // partition的思想
    pub fn exchange(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {
            return nums;
        }
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            while left < right && nums[left] % 2 == 1 {
                left += 1;
            }
            while left < right && nums[right] % 2 == 0 {
                right -= 1;
            }
            if left < right {
                nums.swap(left, right);
            }
        }
        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        println!(
            "{:?}",
            Solution::exchange(vec![1, 2, 3, 3, 4, 5, 6, 7, 7, 8])
        );
    }
}
