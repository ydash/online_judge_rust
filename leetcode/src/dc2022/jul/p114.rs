// 114. Flatten Binary Tree to Linked List

use std::cell::RefCell;
use std::rc::Rc;

use crate::util::TreeNode;
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        root.clone().map(|v| Self::flatten_aux(v));
    }

    fn flatten_aux(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        let node = Rc::clone(&root);
        let mut node = node.borrow_mut();
        let left = node.left.take();
        let right = node.right.clone();
        match (left, right) {
            (None, None) => root,
            (None, Some(r)) => Self::flatten_aux(r),
            (Some(l), None) => {
                node.right = Some(Rc::clone(&l));
                Self::flatten_aux(l)
            }
            (Some(l), Some(r)) => {
                node.right = Some(Rc::clone(&l));
                let left_last = Self::flatten_aux(l);
                left_last.borrow_mut().right = Some(Rc::clone(&r));
                Self::flatten_aux(r)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        Solution::flatten(&mut root.clone());
        let expected = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(expected, root)
    }

    #[test]
    fn test_case_02() {
        let root = None;
        Solution::flatten(&mut root.clone());
        let expected = None;
        assert_eq!(expected, root)
    }

    #[test]
    fn test_case_03() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        })));
        Solution::flatten(&mut root.clone());
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                        }))),
                    }))),
                }))),
            }))),
        })));
        assert_eq!(expected, root)
    }
}
