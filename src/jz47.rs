struct Solution;

impl Solution {
    // DP: max_price[i][j] = max(max_price[i][j], max_price[i - 1][j] + grid[i][j], max_price[i][j - 1] + grid[i][j])
    // time: O(mn)
    // space: O(mn) -> O(min(m,n)) 对于写题目来说可以不做这个优化
    // 若使用dfs，则 time: O(2^(m+2))
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut max_price = vec![vec![0; n]; m];
        max_price[0][0] = grid[0][0];
        for i in 0..m {
            for j in 0..n {
                if i > 0 {
                    max_price[i][j] = max(max_price[i][j], max_price[i - 1][j] + grid[i][j]);
                }
                if j > 0 {
                    max_price[i][j] = max(max_price[i][j], max_price[i][j - 1] + grid[i][j]);
                }
            }
        }
        max_price[m - 1][n - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::max_value(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            12
        );
        assert_eq!(Solution::max_value(vec![vec![1, 3, 1, 100]]), 105);
    }
}
