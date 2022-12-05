// 876. Middle of the Linked List

use crate::util::ListNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.clone();
        let mut fast = head.and_then(|n| n.next);
        while fast.is_some() {
            slow = slow.and_then(|n| n.next);
            fast = fast.and_then(|n| n.next).and_then(|n| n.next);
        }
        slow
    }
}

#[cfg(test)]
mod tests {
    use crate::util::ListNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let head = ListNode::from(vec![1, 2, 3, 4, 5]);
        let result = Solution::middle_node(head);
        assert_eq!(ListNode::from(vec![3, 4, 5]), result)
    }

    #[test]
    fn test_case_02() {
        let head = ListNode::from(vec![1, 2, 3, 4, 5, 6]);
        let result = Solution::middle_node(head);
        assert_eq!(ListNode::from(vec![4, 5, 6]), result)
    }
}
