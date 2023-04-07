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
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut p = &mut dummy;
        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                let tmp = l1.as_mut().unwrap().next.take();
                p.next = l1;
                l1 = tmp;
            } else {
                let tmp = l2.as_mut().unwrap().next.take();
                p.next = l2;
                l2 = tmp;
            }
            p = p.next.as_mut().unwrap();
        }
        if l1.is_none() {
            p.next = l2;
        } else if l2.is_none() {
            p.next = l1;
        }

        dummy.next.take()
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
        let list_1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let list_2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        assert_eq!(
            list_print(&Solution::merge_two_lists(list_1, list_2)),
            vec![1, 1, 2, 3, 4, 4]
        );
    }
}
