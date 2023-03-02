struct Solution;

impl Solution {
    // time: O(n)
    // space: O(1)
    pub fn first_uniq_char(s: String) -> char {
        let mut map = vec![0; 26];
        for c in s.chars() {
            map[c as usize - 'a' as usize] += 1;
        }
        for c in s.chars() {
            if map[c as usize - 'a' as usize] == 1 {
                return c;
            }
        }
        ' '
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::first_uniq_char("abaccdeff".to_owned()), 'b');
        assert_eq!(Solution::first_uniq_char("".to_owned()), ' ');
    }
}
