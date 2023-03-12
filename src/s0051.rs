struct Solution;

impl Solution {
    // N-皇后
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut rst = vec![];
        let n = n as usize;
        let mut chessboard = vec![vec!['.'; n]; n];
        let mut col_check = vec![false; n];
        let mut ldiag_check = vec![false; 2 * n - 1];
        let mut rdiag_check = vec![false; 2 * n - 1];
        Self::dfs(
            &mut chessboard,
            0,
            n,
            &mut col_check,
            &mut ldiag_check,
            &mut rdiag_check,
            &mut rst,
        );

        rst
    }

    fn dfs(
        chessboard: &mut Vec<Vec<char>>,
        row: usize,
        n: usize,
        col_check: &mut Vec<bool>,
        ldiag_check: &mut Vec<bool>,
        rdiag_check: &mut Vec<bool>,
        rst: &mut Vec<Vec<String>>,
    ) {
        let chessboard_to_rst_format = |chessboard: &Vec<Vec<char>>| {
            let mut rst = vec![];
            for i in 0..chessboard.len() {
                rst.push(chessboard[i].iter().collect::<String>());
            }
            rst
        };
        if row == n {
            rst.push(chessboard_to_rst_format(chessboard));
            return;
        }
        for col in 0..n {
            if col_check[col] || ldiag_check[n - row + col - 1] || rdiag_check[row + col] {
                continue;
            }
            chessboard[row][col] = 'Q';
            col_check[col] = true;
            ldiag_check[n - row + col - 1] = true;
            rdiag_check[row + col] = true;
            Self::dfs(
                chessboard,
                row + 1,
                n,
                col_check,
                ldiag_check,
                rdiag_check,
                rst,
            );
            chessboard[row][col] = '.';
            col_check[col] = false;
            ldiag_check[n - row + col - 1] = false;
            rdiag_check[row + col] = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("queen_num = 4:\n{:?}", Solution::solve_n_queens(4));
        println!("queen_num = 1:\n{:?}", Solution::solve_n_queens(1));
    }
}
