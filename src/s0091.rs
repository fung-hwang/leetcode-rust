struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars_parse_to_num =
            |a: char, b: char| (a as usize - '0' as usize) * 10 + (b as usize - '0' as usize);
        let n = s.len();
        if n == 0 {
            return 0;
        }
        let chars = s.chars().collect::<Vec<_>>();
        let mut dp = vec![0; n + 1];
        dp[0] = 1; // 这里需要想一下
        for i in 0..n {
            // 1
            if chars[i] >= '1' && chars[i] <= '9' {
                dp[i + 1] += dp[i];
            }
            // 2
            if i > 0 {
                let two_digit = chars_parse_to_num(chars[i - 1], chars[i]);
                if two_digit >= 10 && two_digit <= 26 {
                    dp[i + 1] += dp[i - 1];
                }
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::num_decodings("226".to_owned()), 3);
        assert_eq!(Solution::num_decodings("06".to_owned()), 0);
        assert_eq!(Solution::num_decodings("11106".to_owned()), 2);
        assert_eq!(Solution::num_decodings("11177".to_owned()), 5);
        assert_eq!(Solution::num_decodings("001726".to_owned()), 0);
    }
}
