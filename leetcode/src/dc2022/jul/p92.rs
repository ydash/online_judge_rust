use crate::util::ListNode;

// 92. Reverse Linked List II
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut stack = vec![];
        let mut current = head;
        for _ in 1..left {
            for n in current.iter() {
                stack.push(n.val);
            }
            current = current.and_then(|n| n.next);
        }
        let mut tmp = vec![];
        for _ in left..=right {
            for n in current.iter() {
                tmp.push(n.val);
            }
            current = current.and_then(|n| n.next);
        }
        while let Some(v) = tmp.pop() {
            stack.push(v);
        }
        while current.is_some() {
            for n in current.iter() {
                stack.push(n.val);
            }
            current = current.and_then(|n| n.next);
        }

        let mut result = None;
        while let Some(v) = stack.pop() {
            result = Some(Box::new(ListNode {
                val: v,
                next: result,
            }));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::util::ListNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::reverse_between(Some(Box::new(ListNode::new(1))), 1, 1);
        assert_eq!(Some(Box::new(ListNode::new(1))), result);
    }

    #[test]
    fn test_case_02() {
        let result = Solution::reverse_between(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode::new(5))),
                        })),
                    })),
                })),
            })),
            2,
            4,
        );
        assert_eq!(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode::new(5))),
                        })),
                    }))
                }))
            })),
            result
        );
    }

    #[test]
    fn test_case_03() {
        let result = Solution::reverse_between(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode::new(5))),
                        })),
                    })),
                })),
            })),
            1,
            5,
        );
        assert_eq!(
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode::new(1))),
                        })),
                    }))
                }))
            })),
            result
        );
    }
}
