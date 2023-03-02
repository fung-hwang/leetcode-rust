struct Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        // let s1 = &s[0..n as usize];
        // let s2 = &s[n as usize..];
        // s2.to_string() + s1

        let mut chars: Vec<char> = s.chars().collect();
        Self::reverse(&mut chars, 0, (n - 1) as usize);
        Self::reverse(&mut chars, n as usize, s.len() - 1);
        chars.reverse();
        chars.into_iter().collect()
    }

    fn reverse(chars: &mut Vec<char>, mut start: usize, mut end: usize) {
        while start < end {
            chars.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse_left_words("abcXYZdef".to_owned(), 3),
            "XYZdefabc".to_owned()
        );
        assert_eq!(
            Solution::reverse_left_words("a".to_owned(), 1),
            "a".to_owned()
        );
    }
}
