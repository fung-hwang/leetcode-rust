struct Solution;

impl Solution {
    // 也可以考虑用栈代替递归
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
            if i < 0
                || i as usize >= grid.len()
                || j < 0
                || j as usize >= grid[0].len()
                || grid[i as usize][j as usize] == 0
            {
                return 0;
            }
            grid[i as usize][j as usize] = 0;
            let mut sum = 1;
            let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
            for d in directions {
                sum += dfs(grid, i + d.0, j + d.1);
            }
            sum
        }
        let mut max_sum = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                max_sum = std::cmp::max(max_sum, dfs(&mut grid, i as i32, j as i32));
            }
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );
        assert_eq!(Solution::max_area_of_island(vec![vec![1]]), 1);
    }
}
