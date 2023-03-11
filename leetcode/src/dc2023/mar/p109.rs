// 109. Convert Sorted List to Binary Search Tree

use std::{cell::RefCell, rc::Rc};

use crate::util::{ListNode, TreeNode};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn aux(head: &mut Option<Box<ListNode>>, length: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if length == 0 {
                None
            } else {
                let left = aux(head, length / 2);
                if let Some(x) = head {
                    let val = x.val;
                    *head = x.next.take();
                    let right = aux(head, length - length / 2 - 1);
                    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
                } else {
                    None
                }
            }
        }
        let length = {
            let mut acc = 0;
            let mut current = &head;
            while let Some(x) = current {
                acc += 1;
                current = &x.next;
            }
            acc
        };
        aux(&mut head, length)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::{ListNode, TreeNode};

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::sorted_list_to_bst(ListNode::from(vec![-10, -3, 0, 5, 9]));
        assert_eq!(
            TreeNode::from(vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)]),
            result
        )
    }

    #[test]
    fn test_case_02() {
        let result = Solution::sorted_list_to_bst(None);
        assert_eq!(None, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::sorted_list_to_bst(ListNode::from(vec![0]));
        assert_eq!(TreeNode::from(vec![Some(0)]), result)
    }
}
