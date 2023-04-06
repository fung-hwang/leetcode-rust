// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    // 递归
    // pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
    //     let mut rst = vec![];
    //     Self::auxiliary(&mut rst, &head);
    //     rst
    // }

    // fn auxiliary(rst: &mut Vec<i32>, p: &Option<Box<ListNode>>) {
    //     match p {
    //         None => return,
    //         Some(node) => {
    //             Self::auxiliary(rst, &node.next);
    //             rst.push(node.val);
    //         }
    //     }
    // }

    // 辅助栈
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut rst = Vec::new();
        let mut p = head;
        while let Some(node) = p {
            rst.push(node.val);
            p = node.next;
        }
        rst.reverse();
        rst
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        assert_eq!(Solution::reverse_print(list), vec![3, 2, 1]);

        let list_2 = None;
        assert_eq!(Solution::reverse_print(list_2), vec![]);
    }
}
