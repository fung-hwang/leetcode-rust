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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut dummy_p = &mut dummy;
        let mut p = head;
        while let Some(mut node_p) = p.take() {
            if let Some(mut node_q) = node_p.next.take() {
                p = node_q.next.take();
                dummy_p.next = Some(node_q);
                dummy_p = dummy_p.next.as_mut().unwrap();
            }
            dummy_p.next = Some(node_p);
            dummy_p = dummy_p.next.as_mut().unwrap();
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_print(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut rst = vec![];
        let mut p = head;
        while let Some(node) = p {
            rst.push(node.val);
            p = &node.next;
        }
        rst
    }

    #[test]
    fn test() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));

        assert_eq!(list_print(&Solution::swap_pairs(list)), vec![2, 1, 4, 3]);

        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        assert_eq!(list_print(&Solution::swap_pairs(list)), vec![2, 1, 3]);
    }
}
