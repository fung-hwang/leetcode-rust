use crate::Solution;
// use std::collections::BTreeSet;
use std::collections::VecDeque;

impl Solution {
    // 你可以假设 k 总是有效的，在输入数组 不为空 的情况下，1 ≤ k ≤ nums.length

    // 使用大顶堆维护窗口中最大值
    // time: O(nlogk) 可以用单调队列优化为O(n)
    // space: O(k)
    // pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    //     let k: usize = k as usize;
    //     let mut rst = vec![];
    //     let mut set = BTreeSet::new();
    //     for i in 0..k {
    //         set.insert(nums[i]);
    //     }
    //     rst.push(*set.last().unwrap());

    //     for i in k..nums.len() {
    //         set.remove(&nums[i - k]);
    //         set.insert(nums[i]);
    //         rst.push(*set.last().unwrap());
    //     }
    //     rst
    // }

    // 使用单调队列维护窗口中最大值
    // 单调队列/单调栈通过维护单调性，可以在O(n)的时间复杂度中处理大小比较的问题
    // time: O(n)
    // space: O(k)
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k: usize = k as usize;
        let mut rst = vec![];
        let mut dq = VecDeque::new();
        for i in 0..nums.len() {
            if let Some(&front) = dq.front() {
                if i >= k && i - k == front {
                    dq.pop_front();
                }
            }
            while let Some(&back) = dq.back() {
                if nums[back] <= nums[i] {
                    dq.pop_back();
                } else {
                    break;
                }
            }
            dq.push_back(i);
            if i >= k - 1 {
                rst.push(nums[*dq.front().unwrap()]);
            }
        }

        rst
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jz59() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(Solution::max_sliding_window(vec![1, 3, -1], 3), vec![3]);
    }
}
