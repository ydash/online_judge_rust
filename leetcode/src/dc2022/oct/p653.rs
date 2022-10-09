// 653. Two Sum IV - Input is a BST

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::util::TreeNode;
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut seen = HashSet::new();
        let mut stack = vec![];
        stack.push(root);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            if let Some(node_ref) = node.clone() {
                let node_ref = node_ref.borrow();
                if seen.contains(&(k - node_ref.val)) {
                    return true;
                }
                seen.insert(node_ref.val);
                stack.push(node_ref.right.clone());
                stack.push(node_ref.left.clone());
            }
        }
        false
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
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        let result = Solution::find_target(root, 9);
        assert!(result)
    }

    #[test]
    fn test_case_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        let result = Solution::find_target(root, 28);
        assert!(!result)
    }
}
