struct Solution;

impl Solution {
    // partition 找数组中的第K个最大元素
    // time: O(n)
    // space: O(1)
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = nums.len() - k as usize;
        fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
            let pivot = nums[low];
            let mut pos = low;
            for i in (low + 1)..=high {
                if nums[i] < pivot {
                    pos += 1;
                    nums.swap(i, pos);
                }
            }
            nums.swap(pos, low);
            pos
        }

        let (mut low, mut high) = (0, nums.len() - 1);
        while low < high {
            let pivot_idx = partition(&mut nums, low, high);
            if pivot_idx > k {
                high = pivot_idx - 1;
            } else if pivot_idx < k {
                low = pivot_idx + 1;
            } else {
                return nums[k];
            }
        }

        // let mut pivot_idx = partition(&mut nums, low, high);
        // while pivot_idx != k {
        //     if pivot_idx > k {
        //         high = pivot_idx - 1;
        //     } else if pivot_idx < k {
        //         low = pivot_idx + 1;
        //     }
        //     pivot_idx = partition(&mut nums, low, high);
        // }
        nums[k]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(Solution::find_kth_largest(vec![1], 1), 1);
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
