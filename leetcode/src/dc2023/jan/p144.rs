// 144. Binary Tree Preorder Traversal

use std::{cell::RefCell, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = vec![root];
        let mut result = vec![];
        while let Some(o) = stack.pop() {
            if let Some(n) = o {
                let node = n.borrow();
                result.push(node.val);
                stack.push(node.right.clone());
                stack.push(node.left.clone());
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
        let result =
            Solution::preorder_traversal(TreeNode::from(vec![Some(1), None, Some(2), Some(3)]));
        assert_eq!(vec![1, 2, 3], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::preorder_traversal(None);
        assert_eq!(Vec::<i32>::new(), result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::preorder_traversal(TreeNode::from(vec![Some(1)]));
        assert_eq!(vec![1], result)
    }
}
