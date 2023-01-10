// 100. Same Tree

use std::{cell::RefCell, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut stack_p = vec![p];
        let mut stack_q = vec![q];
        while let (Some(a), Some(b)) = (stack_p.pop(), stack_q.pop()) {
            match (a, b) {
                (None, None) => (),
                (None, Some(_)) => return false,
                (Some(_), None) => return false,
                (Some(rc_a), Some(rc_b)) => {
                    let node_a = rc_a.borrow();
                    let node_b = rc_b.borrow();
                    if node_a.val != node_b.val {
                        return false;
                    }
                    stack_p.push(node_a.right.clone());
                    stack_p.push(node_a.left.clone());
                    stack_q.push(node_b.right.clone());
                    stack_q.push(node_b.left.clone());
                }
            }
        }
        stack_p.is_empty() && stack_q.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::is_same_tree(
            TreeNode::from(vec![Some(1), Some(2)]),
            TreeNode::from(vec![Some(1), None, Some(2)]),
        );
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_same_tree(
            TreeNode::from(vec![Some(1), Some(2), Some(1)]),
            TreeNode::from(vec![Some(1), Some(1), Some(2)]),
        );
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::is_same_tree(
            TreeNode::from(vec![Some(1), Some(2), Some(3)]),
            TreeNode::from(vec![Some(1), Some(2), Some(3)]),
        );
        assert_eq!(true, result)
    }
}
