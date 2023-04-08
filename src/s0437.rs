// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused)]
struct Solution;

impl Solution {
    // 基本思路是 dfs 遍历树，遍历至每个结点时，计算从 root 到该结点的路径之和 cur_path_sum 减去 其所有祖先的路径之和
    // 通过维护一个 stack/hashmap 记录所有祖先的路径之和
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn path_sum_dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            pre_path_sum: &mut Vec<i64>, // vec 可以换成 hashmap
            target_sum: i32,
            path_cnt: &mut i32,
        ) {
            if root.is_none() {
                return;
            }
            let cur_path_sum =
                *pre_path_sum.last().unwrap() + root.as_ref().unwrap().borrow().val as i64;

            // 注意可能不止一个祖先结点符合要求
            *path_cnt += pre_path_sum.iter().fold(0, |acc, x| {
                if cur_path_sum - target_sum as i64 == *x {
                    acc + 1
                } else {
                    acc
                }
            });

            pre_path_sum.push(cur_path_sum);

            path_sum_dfs(
                &root.as_ref().unwrap().borrow().left,
                pre_path_sum,
                target_sum,
                path_cnt,
            );
            path_sum_dfs(
                &root.as_ref().unwrap().borrow().right,
                pre_path_sum,
                target_sum,
                path_cnt,
            );

            pre_path_sum.pop();
        }

        // 维护一个栈
        let mut pre_path_sum = vec![0]; // 注意初始状态
        let mut path_cnt = 0;
        path_sum_dfs(&root, &mut pre_path_sum, target_sum, &mut path_cnt);
        path_cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // stack 并不一定是一个递增数列
        let ch_1 = Some(Rc::new(RefCell::new(TreeNode::new(-2))));
        // let ch_2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: -1,
            left: None,
            right: ch_1.clone(),
        })));

        assert_eq!(Solution::path_sum(root, -3), 1);
    }

    #[test]
    fn test_2() {
        // 注意可能不止一个祖先结点符合要求
        let ch_1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let ch_2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: ch_1.clone(),
            right: ch_2.clone(),
        })));

        assert_eq!(Solution::path_sum(root, 1), 4);
    }
}
