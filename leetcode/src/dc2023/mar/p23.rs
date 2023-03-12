// 23. Merge k Sorted Lists

use std::{cmp::Reverse, collections::BinaryHeap};

use crate::util::ListNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut pq = BinaryHeap::new();
        for lst in lists {
            if let Some(node) = lst {
                pq.push(Reverse(node));
            }
        }
        let mut dummy_root = ListNode::new(0);
        let mut p = &mut dummy_root;
        while let Some(Reverse(mut node)) = pq.pop() {
            if let Some(next) = node.next.take() {
                pq.push(Reverse(next));
            }
            p.next = Some(node);
            p = p.next.as_mut().unwrap();
        }
        dummy_root.next
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::ListNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::merge_k_lists(vec![
            ListNode::from(vec![1, 4, 5]),
            ListNode::from(vec![1, 3, 4]),
            ListNode::from(vec![2, 6]),
        ]);
        assert_eq!(ListNode::from(vec![1, 1, 2, 3, 4, 4, 5, 6]), result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::merge_k_lists(vec![]);
        assert_eq!(None, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::merge_k_lists(vec![None]);
        assert_eq!(None, result)
    }
}
