struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        use std::cmp::{max, min};
        let (m, n) = (matrix.len(), matrix[0].len());
        if m == 0 || n == 0 {
            return 0;
        }
        // 一种简明处理边界的方法是将 square_side 构建为 [m + 1, n + 1] 大小的二维数组
        let mut square_side = vec![vec![0; n]; m];
        square_side[0] = matrix[0].iter().map(|c| *c as i32 - '0' as i32).collect();
        for i in 1..m {
            square_side[i][0] = matrix[i][0] as i32 - '0' as i32;
        }

        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == '1' {
                    square_side[i][j] = min(
                        min(square_side[i - 1][j], square_side[i - 1][j - 1]),
                        square_side[i][j - 1],
                    ) + 1;
                }
            }
        }
        let mut max_side = 0;
        square_side
            .iter()
            .for_each(|v| v.iter().for_each(|val| max_side = max(*val, max_side)));

        max_side.pow(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            4
        );
        assert_eq!(
            Solution::maximal_square(vec![vec!['0', '1'], vec!['1', '0']]),
            1
        );
    }
}
