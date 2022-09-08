// 94. Binary Tree Inorder Traversal

use std::cell::RefCell;
use std::rc::Rc;

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, acc: &mut Vec<i32>) {
            if node.is_none() {
                return;
            }
            let node = node.unwrap();
            let node = node.borrow();
            dfs(node.left.clone(), acc);
            acc.push(node.val);
            dfs(node.right.clone(), acc);
        }
        let mut result = vec![];
        dfs(root, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
        })));
        let result = Solution::inorder_traversal(root);
        assert_eq!(vec![1, 3, 2], result)
    }

    #[test]
    fn test_case_02() {
        let root = None;
        let result = Solution::inorder_traversal(root);
        assert_eq!(Vec::<i32>::new(), result)
    }

    #[test]
    fn test_case_03() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = Solution::inorder_traversal(root);
        assert_eq!(vec![1], result)
    }
}
