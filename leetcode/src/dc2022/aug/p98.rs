// 98. Validate Binary Search Tree

use std::cell::RefCell;
use std::rc::Rc;

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn validate(
            root: Option<Rc<RefCell<TreeNode>>>,
            lower_bound: Option<i32>,
            upper_bound: Option<i32>,
        ) -> bool {
            if let Some(r) = root {
                let node = r.borrow();
                let v = node.val;
                (lower_bound.is_none() || lower_bound.unwrap() < v)
                    && (upper_bound.is_none() || v < upper_bound.unwrap())
                    && validate(node.left.clone(), lower_bound, Some(v))
                    && validate(node.right.clone(), Some(v), upper_bound)
            } else {
                true
            }
        }
        validate(root, None, None)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        let result = Solution::is_valid_bst(tree);
        assert!(result)
    }

    #[test]
    fn test_case_02() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let result = Solution::is_valid_bst(tree);
        assert!(result)
    }

    #[test]
    fn test_case_03() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        })));
        let result = Solution::is_valid_bst(tree);
        assert!(!result)
    }

    #[test]
    fn test_case_04() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
            }))),
        })));
        let result = Solution::is_valid_bst(tree);
        assert!(!result)
    }

    #[test]
    fn test_case_05() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: i32::MIN,
            left: Some(Rc::new(RefCell::new(TreeNode::new(i32::MIN)))),
            right: None,
        })));
        let result = Solution::is_valid_bst(tree);
        assert!(!result)
    }

    #[test]
    fn test_case_06() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: i32::MAX,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(i32::MAX)))),
        })));
        let result = Solution::is_valid_bst(tree);
        assert!(!result)
    }
}
