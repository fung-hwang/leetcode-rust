struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        use std::collections::HashMap;
        if s.len() == 0 {
            return 0;
        }
        let chars = s.chars().collect::<Vec<_>>();
        let mut max_len = 0;
        let mut max_len_end_with_i = 0;
        let mut substr_status = HashMap::new();
        for i in 0..chars.len() {
            if let Some(last_idx) = substr_status.insert(chars[i], i) {
                // 注意该情况，原因是 map 中可能存在未及时更新的 k/v
                if i - last_idx > max_len_end_with_i {
                    max_len_end_with_i = max_len_end_with_i + 1;
                } else {
                    max_len_end_with_i = i - last_idx;
                }
            } else {
                max_len_end_with_i = max_len_end_with_i + 1;
            }

            max_len = max(max_len, max_len_end_with_i);
            // println!("{:?}", substr_status);
        }

        max_len as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbba".to_owned()),
            3
        );
    }
}
