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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 可以考虑 max_depth 的返回类型为 (bool, i32)
        fn max_depth(root: Option<Rc<RefCell<TreeNode>>>, is_balanced: &mut bool) -> i32 {
            if *is_balanced {
                if root.is_none() {
                    return 0;
                } else {
                    let left_depth =
                        max_depth(root.as_ref().unwrap().borrow().left.clone(), is_balanced);
                    let right_depth =
                        max_depth(root.as_ref().unwrap().borrow().right.clone(), is_balanced);
                    if (left_depth - right_depth).abs() > 1 {
                        *is_balanced = false;
                    }
                    return 1 + std::cmp::max(left_depth, right_depth);
                }
            }
            0
        }

        let mut is_balanced = true;
        max_depth(root, &mut is_balanced);
        is_balanced
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ch_1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let ch_2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: ch_1.clone(),
            right: ch_2.clone(),
        })));

        assert_eq!(Solution::is_balanced(root), true);
    }
}
