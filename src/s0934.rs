use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        let mut q = VecDeque::new();
        // find and mark one island
        let mut island_finded = false;
        for i in 0..n {
            if !island_finded {
                for j in 0..n {
                    if grid[i as usize][j as usize] == 1 {
                        Self::dfs(&mut grid, i as i32, j as i32, &mut q);
                        island_finded = true;
                        break;
                    }
                }
            }
        }

        // const N_2: i32 = 2;
        const N_3: i32 = 3;
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut dis = 0;
        while !q.is_empty() {
            dis += 1;
            let mut num = q.len() as i32;
            while num > 0 {
                if let Some((row, col)) = q.pop_front() {
                    for d in &directions {
                        let i = row + d.0;
                        let j = col + d.1;
                        if i >= 0 && i < n && j >= 0 && j < n {
                            match grid[i as usize][j as usize] {
                                3 | 2 => {
                                    continue;
                                }
                                1 => {
                                    return dis;
                                }
                                0 => {
                                    grid[i as usize][j as usize] = N_3;
                                    q.push_back((i as i32, j as i32));
                                }
                                _ => {}
                            }
                        }
                    }
                }
                num -= 1;
            }
        }
        0
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, q: &mut VecDeque<(i32, i32)>) {
        const N_2: i32 = 2;
        const N_3: i32 = 3;
        if i < 0
            || i as usize >= grid.len()
            || j < 0
            || j as usize >= grid[0].len()
            || grid[i as usize][j as usize] == N_2
            || grid[i as usize][j as usize] == N_3
        {
            return;
        }
        if grid[i as usize][j as usize] == 0 {
            grid[i as usize][j as usize] = 3;
            q.push_back((i, j));
            return;
        }
        grid[i as usize][j as usize] = N_2;
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for d in directions {
            Self::dfs(grid, i + d.0, j + d.1, q);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::shortest_bridge(vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]]),
            2
        );
        assert_eq!(
            Solution::shortest_bridge(vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 1, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1]
            ]),
            1
        );
    }
}
