struct Solution;

impl Solution {
    // Rust处理字符串用 Vec<char> 会好一些，
    // 否则 utf-8 的 String 很难处理，这也导致String无法使用下标s[i]
    // time: O(n)
    // space:: O(n)
    pub fn replace_space(s: String) -> String {
        let mut rst = vec![];
        for c in s.chars() {
            if c == ' ' {
                rst.append(&mut vec!['%', '2', '0']);
            } else {
                rst.push(c);
            }
        }

        rst.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::replace_space("We are happy.".to_owned()),
            "We%20are%20happy.".to_owned()
        );
        assert_eq!(Solution::replace_space(String::new()), String::new());
    }
}
