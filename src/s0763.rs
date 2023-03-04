struct Solution;

impl Solution {
    // 使用贪心的思想寻找每个片段可能的最小结束下标
    pub fn partition_labels(s: String) -> Vec<i32> {
        // 记录每个字母出现的最后下标
        let mut last_appearance_idx = vec![0; 26];
        let chars: Vec<usize> = s.chars().map(|a| a as usize - 'a' as usize).collect();

        for i in 0..chars.len() {
            last_appearance_idx[chars[i]] = i;
        }
        let mut rst = Vec::new();
        let mut part_end = 0;
        let mut part_start = 0;
        for i in 0..chars.len() {
            // if last_appearance_idx[chars[i]] > part_end {
            //     part_end = last_appearance_idx[chars[i]];
            // }
            part_end = std::cmp::max(last_appearance_idx[chars[i]], part_end);
            if i == part_end {
                rst.push((part_end - part_start + 1) as i32);
                part_start = part_end + 1;
            }
        }
        rst
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::partition_labels("abcabd".to_string()), vec![5, 1]);
        assert_eq!(
            Solution::partition_labels("eccbbbbdec".to_string()),
            vec![10]
        );
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
            vec![9, 7, 8]
        )
    }
}
