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
    pub fn delete_node(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut p = &mut head;
        while let Some(node) = p {
            if node.val == val {
                *p = node.next.take();
            }
            // p = &mut p.as_mut().unwrap().next;
            if let Some(node) = p {
                p = &mut node.next;
            }
        }
        head
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
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        assert_eq!(
            list_print(&Solution::delete_node(list.clone(), 2)),
            vec![1, 3]
        );
        assert_eq!(
            list_print(&Solution::delete_node(list.clone(), 1)),
            vec![2, 3]
        );
        assert_eq!(
            list_print(&Solution::delete_node(list.clone(), 3)),
            vec![1, 2]
        );
    }
}
