struct Solution;

impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let len = nums.len();
        let mut tmp = vec![0; len];
        Self::merge_sort(&mut nums, 0, len - 1, &mut tmp)
    }

    fn merge_sort(nums: &mut Vec<i32>, left: usize, right: usize, tmp: &mut Vec<i32>) -> i32 {
        // 左闭右闭
        if left == right {
            return 0;
        }
        let mid = left + (right - left) / 2;
        let mut cnt =
            Self::merge_sort(nums, left, mid, tmp) + Self::merge_sort(nums, mid + 1, right, tmp);
        let (mut i, mut j, mut k) = (left, mid + 1, left);

        while i <= mid || j <= right {
            // 尤其需注意此处条件的写法（易出错）
            if j > right || i <= mid && nums[i] <= nums[j] {
                tmp[k] = nums[i];
                i += 1;
                cnt += (j - mid - 1) as i32;
            } else {
                tmp[k] = nums[j];
                j += 1;
            }
            k += 1;
        }
        for i in left..k {
            nums[i] = tmp[i];
        }

        cnt
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse_pairs(vec![7, 5, 6, 4]), 5);
        assert_eq!(Solution::reverse_pairs(vec![7, 5, 3, 6, 4, 2]), 12);
        assert_eq!(Solution::reverse_pairs(vec![4, 5, 6, 7]), 0);
        assert_eq!(Solution::reverse_pairs(vec![1, 2]), 0);
    }
}
