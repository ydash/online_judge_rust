// 328. Odd Even Linked List

use crate::util::ListNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut dummy_head = ListNode { val: 0, next: head };
        let mut p_odd = dummy_head.next.as_mut().unwrap().as_mut();
        let mut head_of_even = ListNode::new(0);
        let mut p_even = &mut head_of_even;
        while p_odd.next.is_some() {
            p_even.next = p_odd.next.take();
            p_even = p_even.next.as_mut().unwrap().as_mut();
            if p_even.next.is_some() {
                p_odd.next = p_even.next.take();
                p_odd = p_odd.next.as_mut().unwrap().as_mut();
            }
        }
        p_odd.next = head_of_even.next;
        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use crate::util::ListNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let head = ListNode::from(vec![1, 2, 3, 4, 5]);
        let result = Solution::odd_even_list(head);
        assert_eq!(ListNode::from(vec![1, 3, 5, 2, 4]), result)
    }

    #[test]
    fn test_case_02() {
        let head = ListNode::from(vec![1, 2, 3, 4]);
        let result = Solution::odd_even_list(head);
        assert_eq!(ListNode::from(vec![1, 3, 2, 4]), result)
    }
}
