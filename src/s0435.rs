struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        // 按区间右侧的大小排序
        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut non_overlapping_intervals_cnt = 0;
        let mut end = i32::MIN;
        for interval in &intervals {
            if interval[0] >= end {
                end = interval[1];
                non_overlapping_intervals_cnt += 1;
            }
        }
        intervals.len() as i32 - non_overlapping_intervals_cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2]]),
            3
        );
    }
}
