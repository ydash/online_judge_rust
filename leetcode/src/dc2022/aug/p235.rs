// 235. Lowest Common Ancestor of a Binary Search Tree

use std::cell::RefCell;
use std::rc::Rc;

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut current = root;
        let v1 = p.unwrap().borrow().val;
        let v2 = q.unwrap().borrow().val;
        let (v1, v2) = (v1.min(v2), v1.max(v2));
        while let Some(rc) = current.clone() {
            let node = rc.borrow();
            if node.val < v1 {
                current = node.right.clone();
            } else if node.val > v2 {
                current = node.left.clone();
            } else {
                return current;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        })));
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: p.clone(),
            right: q.clone(),
        })));
        let result = Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone());
        let expected = root.clone();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_case_02() {
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        })));
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: q.clone(),
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: p.clone(),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            }))),
        })));
        let result = Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone());
        let expected = p.clone();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_case_03() {
        let p = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: q.clone(),
            right: p.clone(),
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
            }))),
            right: expected.clone(),
        })));
        let result = Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone());
        assert_eq!(expected, result);
    }
}
