// 86. Partition List

use crate::util::ListNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy1 = ListNode::new(0);
        let mut dummy2 = ListNode::new(0);

        let mut p1 = &mut dummy1;
        let mut p2 = &mut dummy2;
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                p1.next = Some(node);
                p1 = p1.next.as_mut().unwrap();
            } else {
                p2.next = Some(node);
                p2 = p2.next.as_mut().unwrap();
            }
        }

        p1.next = dummy2.next;
        dummy1.next
    }
}

#[cfg(test)]
mod tests {
    use crate::util::ListNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::partition(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode::new(2))),
                            })),
                        })),
                    })),
                })),
            })),
            3,
        );
        assert_eq!(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 3,
                                next: Some(Box::new(ListNode::new(5))),
                            })),
                        })),
                    })),
                })),
            })),
            result
        )
    }

    #[test]
    fn test_case_02() {
        let result = Solution::partition(Some(Box::new(ListNode::new(5))), 1);
        assert_eq!(Some(Box::new(ListNode::new(5))), result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::partition(Some(Box::new(ListNode::new(5))), 10);
        assert_eq!(Some(Box::new(ListNode::new(5))), result)
    }
}
