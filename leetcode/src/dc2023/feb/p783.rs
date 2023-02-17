// 783. Minimum Distance Between BST Nodes

use std::{cell::RefCell, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MAX;
        let root = root.unwrap();
        let root = root.borrow();
        let mut stack = vec![];
        stack.push((root.val, root.val, root.left.clone()));
        stack.push((root.val, root.val, root.right.clone()));
        while let Some((min_val, max_val, node)) = stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                stack.push((min_val, node.val, node.left.clone()));
                stack.push((node.val, max_val, node.right.clone()));
                let diff1 = (min_val - node.val).abs();
                let diff2 = (max_val - node.val).abs();
                result = diff1.min(diff2).min(result);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_diff_in_bst(TreeNode::from(vec![
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
        ]));
        assert_eq!(result, 1)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_diff_in_bst(TreeNode::from(vec![
            Some(1),
            Some(0),
            Some(48),
            None,
            None,
            Some(12),
            Some(49),
        ]));
        assert_eq!(result, 1)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_diff_in_bst(TreeNode::from(vec![
            Some(90),
            Some(69),
            None,
            Some(49),
            Some(89),
            None,
            Some(52),
        ]));
        assert_eq!(result, 1)
    }
}
