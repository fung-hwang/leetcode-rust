struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let left = nums.partition_point(|&num| num < target);
        let right = nums.partition_point(|&num| num <= target);
        (right - left) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::search(vec![5, 7, 7, 8, 8, 10], 8), 2);
        assert_eq!(Solution::search(vec![5, 7, 7, 8, 8, 10], 6), 0);
        assert_eq!(Solution::search(vec![7, 7, 7], 7), 3);
    }
}
