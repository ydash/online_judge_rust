// 938. Range Sum of BST

use std::{cell::RefCell, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut result = 0;
        let mut stack = vec![root.clone()];
        while !stack.is_empty() {
            stack.pop().unwrap().map(|n| {
                let n = n.borrow();
                if n.val < low {
                    stack.push(n.right.clone());
                } else if n.val > high {
                    stack.push(n.left.clone());
                } else {
                    result += n.val;
                    stack.push(n.left.clone());
                    stack.push(n.right.clone());
                }
            });
        }
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
            val: 10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(18)))),
            }))),
        })));
        let result = Solution::range_sum_bst(root, 5, 15);
        assert_eq!(50, result)
    }
}
