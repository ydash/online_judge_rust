// 1026. Maximum Difference Between Node and Ancestor

use std::{cell::RefCell, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32, result: &mut i32) {
            if let Some(n) = node {
                let n = n.borrow();
                let min = min.min(n.val);
                let max = max.max(n.val);
                *result = (max - min).max(*result);
                if n.left.is_some() {
                    dfs(n.left.clone(), min, max, result);
                }
                if n.right.is_some() {
                    dfs(n.right.clone(), min, max, result);
                }
            }
        }
        let mut result = 0;
        root.clone().map(|n| {
            let n = n.borrow();
            dfs(root, n.val, n.val, &mut result);
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let root = TreeNode::from(vec![
            Some(8),
            Some(3),
            Some(10),
            Some(1),
            Some(6),
            None,
            Some(14),
            None,
            None,
            Some(4),
            Some(7),
            Some(13),
        ]);
        let result = Solution::max_ancestor_diff(root);
        assert_eq!(7, result)
    }

    #[test]
    fn test_case_02() {
        let root = TreeNode::from(vec![Some(1), None, Some(2), None, Some(0), Some(3)]);
        let result = Solution::max_ancestor_diff(root);
        assert_eq!(3, result)
    }
}
