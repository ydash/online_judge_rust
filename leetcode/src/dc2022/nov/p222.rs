// 222. Count Complete Tree Nodes

use std::{cell::RefCell, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn depth(node: Option<Rc<RefCell<TreeNode>>>) -> usize {
            let mut dep = 0;
            let mut current = node;
            while let Some(x) = current.clone() {
                dep += 1;
                current = x.borrow().left.clone();
            }
            dep
        }
        let mut result = 0;
        let mut current = root;
        while let Some(x) = current.clone() {
            let x = x.borrow();
            let ld = depth(x.left.clone());
            let rd = depth(x.right.clone());
            result += 1 << rd;
            current = if ld == rd {
                x.right.clone()
            } else {
                x.left.clone()
            };
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
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: None,
            }))),
        })));
        let result = Solution::count_nodes(root);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_02() {
        let root = None;
        let result = Solution::count_nodes(root);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_03() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = Solution::count_nodes(root);
        assert_eq!(1, result)
    }
}
