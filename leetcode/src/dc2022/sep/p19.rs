// 19. Remove Nth Node From End of List

use crate::util::ListNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { next: head, val: 0 }));
        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();
        for _ in 0..=n {
            fast = fast.and_then(|node| node.next);
        }
        while fast.is_some() {
            fast = fast.and_then(|node| node.next);
            slow = slow.and_then(|node| node.next.as_mut());
        }
        slow.map(|mut node| {
            let next = node.next.take();
            node.next = next.and_then(|next| next.next);
        });
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use crate::util::ListNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let head = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode {
                        next: Some(Box::new(ListNode::new(5))),
                        val: 4,
                    })),
                    val: 3,
                })),
                val: 2,
            })),
            val: 1,
        }));
        let result = Solution::remove_nth_from_end(head, 2);
        let expected = Some(Box::new(ListNode {
            next: Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode::new(5))),
                    val: 3,
                })),
                val: 2,
            })),
            val: 1,
        }));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_case_02() {
        let head = Some(Box::new(ListNode::new(1)));
        let result = Solution::remove_nth_from_end(head, 1);
        assert_eq!(None, result)
    }
}
