struct Solution;

impl Solution {
    // 参考解法（可能是剑指offer的本意，但不符合leetcode的题目要求）：
    // 先翻转每个单词，再翻转整个字符串。即 (a^r b^r)^r = b a 。
    // 题目应该有一个隐含条件，就是不能用额外的空间。正确的参数应该是 Vec<String> 而非 String。
    //
    // leetcode要求将两个单词间有多余的空格减少到只含一个，因此以上方法是不可以的。
    pub fn reverse_words(s: String) -> String {
        s.trim()
            // .split(char::is_whitespace)
            // .filter(|item| *item != "")
            .split_ascii_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
        // words.reverse();
        // words.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_owned()),
            "blue is sky the".to_owned()
        );
        assert_eq!(
            Solution::reverse_words("  hello   world!  ".to_owned()),
            "world! hello".to_owned()
        );
    }
}
