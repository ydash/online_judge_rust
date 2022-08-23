// 234. Palindrome Linked List

use crate::util::ListNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut current = head.as_ref();
        let mut length = 0;
        while let Some(node) = current {
            current = node.next.as_ref();
            length += 1;
        }
        let mid = length / 2;
        let mut front_rev: Option<Box<ListNode>> = None;
        let mut back = head;
        for _ in 0..mid {
            let tmp = back.as_mut().and_then(|node| {
                let next = node.next.take();
                node.next = front_rev;
                next
            });
            front_rev = back;
            back = tmp;
        }
        let mut front_rev = front_rev.as_ref();
        let mut back = back.as_ref();
        if length % 2 != 0 {
            back.map(|n| back = n.next.as_ref());
        }
        for _ in 0..mid {
            let fp = front_rev.unwrap();
            let bp = back.unwrap();
            if fp.val != bp.val {
                return false;
            }
            front_rev = fp.next.as_ref();
            back = bp.next.as_ref();
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::util::ListNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(1))),
            })),
        }));
        let result = Solution::is_palindrome(head);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(2))),
            })),
        }));
        let result = Solution::is_palindrome(head);
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(2))),
        }));
        let result = Solution::is_palindrome(head);
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_04() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(1))),
        }));
        let result = Solution::is_palindrome(head);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_05() {
        let head = Some(Box::new(ListNode::new(1)));
        let result = Solution::is_palindrome(head);
        assert_eq!(true, result)
    }
}
