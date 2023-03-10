struct Solution;

impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut visited = vec![vec![false; n as usize]; m as usize];
        Self::dfs(&mut visited, m, n, 0, 0, k)
    }

    fn dfs(visited: &mut Vec<Vec<bool>>, m: i32, n: i32, i: i32, j: i32, k: i32) -> i32 {
        if i < 0
            || i >= m
            || j < 0
            || j >= n
            || visited[i as usize][j as usize]
            || !Self::local_check(i, j, k)
        {
            return 0;
        }
        let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        visited[i as usize][j as usize] = true;
        let mut rst = 1;
        for d in directions {
            rst += Self::dfs(visited, m, n, i + d.0, j + d.1, k);
        }
        rst
    }

    // 可以考虑写成闭包放在dfs函数中
    fn local_check(i: i32, j: i32, k: i32) -> bool {
        fn digit_sum(mut num: i32) -> i32 {
            let mut sum = 0;
            while num != 0 {
                sum += num % 10;
                num /= 10;
            }
            sum
        }
        digit_sum(i) + digit_sum(j) <= k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::moving_count(2, 3, 1), 3);
        assert_eq!(Solution::moving_count(3, 1, 0), 1);
    }
}
