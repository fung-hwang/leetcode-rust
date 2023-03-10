struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::dfs(
                    &board,
                    &mut visited,
                    i as i32,
                    j as i32,
                    &word.chars().collect(),
                    0,
                ) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        i: i32,
        j: i32,
        word: &Vec<char>,
        word_idx: usize,
    ) -> bool {
        if i < 0
            || i as usize >= board.len()
            || j < 0
            || j as usize >= board[0].len()
            || visited[i as usize][j as usize]
            || board[i as usize][j as usize] != word[word_idx]
        {
            return false;
        }
        if word_idx == word.len() - 1 {
            return true;
        }
        visited[i as usize][j as usize] = true;
        let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        for d in directions {
            if Self::dfs(board, visited, i + d.0, j + d.1, word, word_idx + 1) {
                return true;
            }
        }
        visited[i as usize][j as usize] = false;

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['C', 'D', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCD".to_string()
            ),
            false
        );
        assert_eq!(Solution::exist(vec![vec!['a']], "a".to_string()), true);
    }
}
