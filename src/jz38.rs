struct Solution;

impl Solution {
    // 一种更优的解法是“下一个排列”，基本操作是 swap
    pub fn permutation(s: String) -> Vec<String> {
        let mut chars = s.chars().collect::<Vec<_>>();
        chars.sort_unstable();
        let mut rst = vec![];
        let mut visited = vec![false; chars.len()];
        Self::dfs(&mut rst, &chars, &mut visited, &mut String::new());
        rst
    }

    fn dfs(rst: &mut Vec<String>, chars: &Vec<char>, visited: &mut Vec<bool>, s: &mut String) {
        if s.len() == chars.len() {
            rst.push(s.clone());
        }
        for i in 0..chars.len() {
            if visited[i] || i > 0 && chars[i] == chars[i - 1] && !visited[i - 1] {
                continue;
            }
            visited[i] = true;
            s.push(chars[i]);
            Self::dfs(rst, chars, visited, s);
            s.pop();
            visited[i] = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", Solution::permutation("aba".to_string()));
        println!("{:?}", Solution::permutation("a".to_string()));
    }
}
