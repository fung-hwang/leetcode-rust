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
    pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut p = &head;
        let mut q = &head;
        let mut cnt = 0;
        while cnt != k {
            match q {
                Some(node) => {
                    q = &node.next;
                    cnt += 1;
                }
                None => return None,
            }
        }
        // while let Some(node_q) = q {
        //     q = &node_q.next;
        //     if let Some(node_p) = p {
        //         p = &node_p.next;
        //     }
        // }
        while q.is_some() {
            q = &q.as_ref().unwrap().next;
            p = &p.as_ref().unwrap().next;
        }

        p.to_owned()
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
        assert_eq!(list_print(&Solution::get_kth_from_end(list, 2)), vec![2, 3]);

        let list = Some(Box::new(ListNode { val: 1, next: None }));
        assert_eq!(list_print(&Solution::get_kth_from_end(list, 1)), vec![1]);
    }
}
