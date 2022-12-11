// 124. Binary Tree Maximum Path Sum

use std::{cell::RefCell, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn search(node: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
            if let Some(node) = node {
                let node_ref = node.borrow();
                let left_max_path = search(node_ref.left.clone(), result);
                let right_max_path = search(node_ref.right.clone(), result);
                let path_a = node_ref.val + left_max_path;
                let path_b = node_ref.val + right_max_path;
                let path_c = node_ref.val + left_max_path + right_max_path;
                let max_path = node_ref.val.max(path_a.max(path_b));
                let max_sum = max_path.max(path_c.max(*result));
                *result = max_sum.max(*result);
                max_path
            } else {
                0
            }
        }
        let mut result = i32::MIN;
        search(root, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let root = TreeNode::from(vec![Some(1), Some(2), Some(3)]);
        let result = Solution::max_path_sum(root);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_02() {
        let root = TreeNode::from(vec![
            Some(-10),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let result = Solution::max_path_sum(root);
        assert_eq!(42, result)
    }

    #[test]
    fn test_case_03() {
        let root = TreeNode::from(vec![Some(-1)]);
        let result = Solution::max_path_sum(root);
        assert_eq!(-1, result)
    }

    #[test]
    fn test_case_04() {
        let root = TreeNode::from(vec![Some(1), Some(-2), Some(-3)]);
        let result = Solution::max_path_sum(root);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_05() {
        let root = TreeNode::from(vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ]);
        let result = Solution::max_path_sum(root);
        assert_eq!(48, result)
    }
}
