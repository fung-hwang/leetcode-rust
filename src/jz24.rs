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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        // （法2）头插法
        let mut cur = head;
        let mut head = None;
        while let Some(mut node) = cur {
            // 此时 cur.node 所有权已 move（允许结构体中部分成员所有权移动，其他成员仍可以使用）
            let next = node.next.take();
            node.next = head;
            head = Some(node);
            cur = next;
        }
        head

        // （法1）依次逆转指向
        // let mut pre = None;
        // let mut cur = head;
        // while let Some(mut node) = cur {
        //     let next = node.next.take();
        //     node.next = pre;
        //     pre = Some(node);
        //     cur = next;
        // }
        // pre
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

        assert_eq!(list_print(&Solution::reverse_list(list)), vec![3, 2, 1]);

        let list = Some(Box::new(ListNode { val: 1, next: None }));
        assert_eq!(list_print(&Solution::reverse_list(list)), vec![1]);
    }
}
