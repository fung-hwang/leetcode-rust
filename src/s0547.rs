struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        if is_connected.len() == 0 {
            return 0;
        }
        let mut cnt = 0;
        let mut visited = vec![false; is_connected.len()];
        for i in 0..visited.len() {
            if !visited[i] {
                cnt += 1;
                Self::dfs(&is_connected, &mut visited, i);
            }
        }
        cnt
    }

    fn dfs(is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>, i: usize) {
        if !visited[i] {
            visited[i] = true;
            for j in 0..visited.len() {
                if is_connected[i][j] == 1 {
                    Self::dfs(is_connected, visited, j);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
            2
        );
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
        assert_eq!(Solution::find_circle_num(vec![vec![1]]), 1);
    }
}
