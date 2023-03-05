struct Solution;

impl Solution {
    // 基本思路：旋转数组变体，二分法
    // time: O(lgn)
    // space: O(1)
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        let (mut low, mut high) = (0, numbers.len() - 1);
        while low < high {
            let mid = low + (high - low) / 2;
            if numbers[mid] < numbers[high] {
                high = mid;
            } else if numbers[mid] > numbers[high] {
                low = mid + 1;
            } else {
                high -= 1;
            }
        }
        numbers[low]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_array(vec![2, 2, 2, 0, 1]), 0);
        assert_eq!(Solution::min_array(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::min_array(vec![1, 1, 1]), 1);
    }
}
