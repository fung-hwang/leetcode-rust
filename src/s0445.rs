struct Solution;

impl Solution {
    // 贪心之处在于每次寻找最小能满足某个孩子胃口的饼干
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let (mut child, mut cookie) = (0, 0);
        while child < g.len() && cookie < s.len() {
            if g[child] <= s[cookie] {
                child += 1;
            }
            cookie += 1;
        }
        child as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_content_children(vec![1, 2, 3], vec![3]), 1);
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
        assert_eq!(Solution::find_content_children(vec![1], vec![]), 0);
    }
}
