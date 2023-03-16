struct Solution;

use std::collections::VecDeque;

// 关于在 leetcode 上超时的问题，主要是因为可以构造特殊解使得所有 bfs 方法（单向/双向）的潜在路径数量呈指数增长
// 正确的方法是：bfs 找最短路径的长度，再从 end_word 开始 dfs 搜索路径
// 思路参考：https://leetcode.cn/problems/word-ladder-ii/solution/yi-ge-shu-ju-tuan-mie-suo-you-bfsdai-ma-k5i5y/
impl Solution {
    // 该解题方法会超时
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        mut word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let end_word_idx = match word_list.iter().position(|w| *w == end_word) {
            Some(idx) => idx,
            None => return Vec::new(),
        };

        let begin_word_idx = match word_list.iter().position(|w| *w == begin_word) {
            Some(idx) => idx,
            None => {
                word_list.push(begin_word.clone());
                word_list.len() - 1
            }
        };

        let mut next = vec![Vec::new(); word_list.len()];
        let mut word_level = vec![-1; word_list.len()];
        word_level[begin_word_idx] = 0;

        // bfs
        let mut q = VecDeque::new();
        q.push_back(begin_word_idx);
        let mut cur_level = 0;
        while !q.is_empty() {
            let mut level_cnt = q.len();
            while level_cnt > 0 {
                if let Some(s1) = q.pop_front() {
                    for (idx_2, s2) in word_list.iter().enumerate() {
                        if (word_level[idx_2] == -1 || word_level[idx_2] == cur_level + 1)
                            && Self::check(&word_list[s1], s2)
                        {
                            if word_level[idx_2] == -1 {
                                q.push_back(idx_2);
                            }
                            word_level[idx_2] = cur_level + 1;
                            next[s1].push(idx_2);

                            // println!(
                            //     "{:?}  {:?}",
                            //     word_list[s1],
                            //     next[s1]
                            //         .iter()
                            //         .map(|s1| word_list[*s1].clone())
                            //         .collect::<Vec<_>>()
                            // );
                        }
                    }
                }
                level_cnt -= 1;
            }
            // 找到 end_word
            if word_level[end_word_idx] != -1 {
                break;
            }
            cur_level += 1;
        }
        // println!("{:?}", next);

        let mut rst: Vec<Vec<String>> = vec![];
        Self::dfs(
            begin_word_idx,
            end_word_idx,
            &word_list,
            &next,
            &mut vec![begin_word],
            &mut rst,
        );
        rst
    }

    fn check(s1: &String, s2: &String) -> bool {
        s1.chars()
            .zip(s2.chars())
            .filter(|(c1, c2)| c1 != c2)
            .count()
            == 1
    }

    fn dfs(
        cur_word_idx: usize,
        end_word_idx: usize,
        word_list: &Vec<String>,
        next: &Vec<Vec<usize>>,
        path: &mut Vec<String>,
        rst: &mut Vec<Vec<String>>,
    ) {
        if cur_word_idx == end_word_idx {
            rst.push(path.clone());
            return;
        }
        for &i in &next[cur_word_idx] {
            path.push(word_list[i].clone());
            Self::dfs(i, end_word_idx, word_list, next, path, rst);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_ladders(
                "hit".to_owned(),
                "cog".to_owned(),
                vec![
                    "hot".to_owned(),
                    "dot".to_owned(),
                    "dog".to_owned(),
                    "lot".to_owned(),
                    "log".to_owned(),
                    "cog".to_owned()
                ]
            ),
            vec![
                vec![
                    "hit".to_owned(),
                    "hot".to_owned(),
                    "dot".to_owned(),
                    "dog".to_owned(),
                    "cog".to_owned()
                ],
                vec![
                    "hit".to_owned(),
                    "hot".to_owned(),
                    "lot".to_owned(),
                    "log".to_owned(),
                    "cog".to_owned()
                ]
            ]
        );
        assert_eq!(
            Solution::find_ladders(
                "a".to_owned(),
                "c".to_owned(),
                vec!["a".to_owned(), "b".to_owned(), "c".to_owned(),]
            ),
            vec![vec!["a".to_owned(), "c".to_owned()]]
        );
    }
}
