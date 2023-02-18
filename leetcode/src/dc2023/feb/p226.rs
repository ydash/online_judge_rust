// 226. Invert Binary Tree

use std::{cell::RefCell, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(x) = root.clone() {
            let mut x = x.borrow_mut();
            let left = Self::invert_tree(x.left.take());
            let right = Self::invert_tree(x.right.take());
            x.left = right;
            x.right = left;
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::invert_tree(TreeNode::from(vec![Some(2), Some(1), Some(3)]));
        assert_eq!(TreeNode::from(vec![Some(2), Some(3), Some(1)]), result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::invert_tree(TreeNode::from(vec![
            Some(4),
            Some(2),
            Some(7),
            None,
            Some(3),
            Some(6),
        ]));
        assert_eq!(
            TreeNode::from(vec![Some(4), Some(7), Some(2), None, Some(6), Some(3)]),
            result
        )
    }

    #[test]
    fn test_case_03() {
        let result = Solution::invert_tree(None);
        assert_eq!(None, result)
    }
}
