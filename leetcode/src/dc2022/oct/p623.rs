// 623. Add One Row to Tree

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })));
        }

        let mut q = VecDeque::new();
        q.push_back(root.clone().unwrap());
        for _ in 1..depth - 1 {
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let node = node.borrow();
                if let Some(l) = node.left.clone() {
                    q.push_back(l);
                }
                if let Some(r) = node.right.clone() {
                    q.push_back(r);
                }
            }
        }
        while let Some(node) = q.pop_front() {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            let new_left = Some(Rc::new(RefCell::new(TreeNode {
                val,
                left,
                right: None,
            })));
            let new_right = Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right,
            })));
            node.left = new_left;
            node.right = new_right;
        }
        root
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
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: None,
            }))),
        })));
        let result = Solution::add_one_row(root, 1, 2);
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(expected, result)
    }

    #[test]
    fn test_case_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = Solution::add_one_row(root, 2, 1);
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: None,
        })));
        assert_eq!(expected, result)
    }
}
