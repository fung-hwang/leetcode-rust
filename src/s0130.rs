use core::borrow;

struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        for i in [0, board.len() - 1] {
            for j in 0..board[0].len() {
                Self::dfs(i as i32, j as i32, board);
            }
        }
        for j in [0, board[0].len() - 1] {
            for i in 0..board.len() {
                Self::dfs(i as i32, j as i32, board);
            }
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
                if board[i][j] == 'A' {
                    board[i][j] = 'O';
                }
            }
        }
    }

    fn dfs(i: i32, j: i32, board: &mut Vec<Vec<char>>) {
        if i < 0
            || i as usize >= board.len()
            || j < 0
            || j as usize >= board[0].len()
            || board[i as usize][j as usize] == 'X'
            || board[i as usize][j as usize] == 'A'
        {
            return;
        }
        board[i as usize][j as usize] = 'A';
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for d in directions {
            Self::dfs(i + d.0, j + d.1, board);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut a = vec![
            vec!['X', 'O', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut a);
        println!("{:?}", &a);
    }
}
