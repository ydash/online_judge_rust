// 1448. Count Good Nodes in Binary Tree
use std::cell::RefCell;
use std::rc::Rc;

use crate::util::TreeNode;
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(current: Option<Rc<RefCell<TreeNode>>>, mut max_val: i32) -> i32 {
            let mut result = 0;
            if let Some(r) = current {
                let node = r.borrow();
                let v = node.val;
                if v >= max_val {
                    max_val = v;
                    result += 1;
                }
                result += dfs(node.left.clone(), max_val);
                result += dfs(node.right.clone(), max_val);
            }
            result
        }
        dfs(root, i32::MIN)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(-1000))));
        let result = Solution::good_nodes(root);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        })));
        let result = Solution::good_nodes(root);
        assert_eq!(4, result)
    }
}
