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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            0
        } else {
            1 + std::cmp::max(
                Self::max_depth(root.as_ref().unwrap().borrow().left.clone()),
                Self::max_depth(root.as_ref().unwrap().borrow().right.clone()),
            )
        }
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

        assert_eq!(Solution::max_depth(root), 2);
    }
}
