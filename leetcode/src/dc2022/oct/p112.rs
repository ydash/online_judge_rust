// 112. Path Sum

use std::cell::RefCell;
use std::rc::Rc;

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn aux(current: Rc<RefCell<TreeNode>>, target_sum: i32, acc: i32) -> bool {
            let current = current.borrow();
            let acc = acc + current.val;
            match (current.left.clone(), current.right.clone()) {
                (None, None) => acc == target_sum,
                (None, Some(left)) => aux(left, target_sum, acc),
                (Some(right), None) => aux(right, target_sum, acc),
                (Some(left), Some(right)) => {
                    aux(left, target_sum, acc) || aux(right, target_sum, acc)
                }
            }
        }
        if root.is_some() {
            aux(root.unwrap(), target_sum, 0)
        } else {
            false
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
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                right: None,
            }))),
        })));
        let result = Solution::has_path_sum(root, 22);
        assert!(result)
    }

    #[test]
    fn test_case_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let result = Solution::has_path_sum(root, 5);
        assert!(!result)
    }

    #[test]
    fn test_case_03() {
        let root = None;
        let result = Solution::has_path_sum(root, 0);
        assert!(!result)
    }
}
