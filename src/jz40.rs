use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    // BinaryHeap: A priority queue implemented with a binary heap. This will be a max-heap.
    // Either core::cmp::Reverse or a custom Ord implementation can be used to make BinaryHeap a min-heap.
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let k: usize = k as usize;
        if k == 0 {
            return vec![];
        }
        let mut max_heap = BinaryHeap::with_capacity(k);

        for i in arr {
            if max_heap.len() < k {
                max_heap.push(i);
            } else {
                if *max_heap.peek().unwrap() > i {
                    max_heap.pop();
                    max_heap.push(i)
                }
            }
        }

        // max_heap.into_iter().collect()
        max_heap.into_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::get_least_numbers(vec![0, 1, 2, 1], 1), vec![0]);
        assert_eq!(Solution::get_least_numbers(vec![2, 1, 3, 1], 2), vec![1, 1]);
    }
}
