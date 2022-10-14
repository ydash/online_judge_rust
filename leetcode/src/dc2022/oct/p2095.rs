// 2095. Delete the Middle Node of a Linked List

use crate::util::ListNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_root = ListNode { val: 0, next: head };
        let mut p = &mut dummy_root;
        let mut length = 0;
        while p.next.is_some() {
            length += 1;
            p = p.next.as_mut().unwrap();
        }
        let mut p = &mut dummy_root;
        for _ in 0..length / 2 {
            p = p.next.as_mut().unwrap();
        }
        p.next = p.next.as_mut().unwrap().next.take();
        dummy_root.next
    }
}

#[cfg(test)]
mod tests {
    use crate::util::ListNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let head = ListNode::from(vec![1, 3, 4, 7, 1, 2, 6]);
        let result = Solution::delete_middle(head);
        assert_eq!(ListNode::from(vec![1, 3, 4, 1, 2, 6]), result)
    }

    #[test]
    fn test_case_02() {
        let head = ListNode::from(vec![2, 1]);
        let result = Solution::delete_middle(head);
        assert_eq!(ListNode::from(vec![2]), result)
    }

    #[test]
    fn test_case_03() {
        let head = ListNode::from(vec![1]);
        let result = Solution::delete_middle(head);
        assert_eq!(None, result)
    }
}
