// 606. Construct String from Binary Tree

use std::cell::RefCell;
use std::rc::Rc;

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "".to_owned();
        }
        let node = root.unwrap();
        let node = node.borrow();
        let v = node.val;
        let left = Self::tree2str(node.left.clone());
        let right = Self::tree2str(node.right.clone());
        if left.is_empty() && right.is_empty() {
            v.to_string()
        } else if left.is_empty() {
            format!("{}()({})", v, right)
        } else if right.is_empty() {
            format!("{}({})", v, left)
        } else {
            format!("{}({})({})", v, left, right)
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
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let result = Solution::tree2str(root);
        assert_eq!("1(2(4))(3)".to_owned(), result)
    }
}
