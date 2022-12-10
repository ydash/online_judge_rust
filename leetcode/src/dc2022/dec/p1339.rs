// 1339. Maximum Product of Splitted Binary Tree

use std::{cell::RefCell, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, sum: i64, result: &mut i64) -> i64 {
            if let Some(n) = node {
                let node_ref = n.borrow();
                let sub_sum = node_ref.val as i64
                    + dfs(node_ref.left.clone(), sum, result)
                    + dfs(node_ref.right.clone(), sum, result);
                *result = (sub_sum * (sum - sub_sum)).max(*result);
                sub_sum
            } else {
                0
            }
        }
        let sum = {
            let mut acc = 0;
            let mut stack = vec![root.clone()];
            while let Some(rc) = stack.pop().and_then(|n| n) {
                let node_ref = rc.borrow();
                acc += node_ref.val;
                if node_ref.left.is_some() {
                    stack.push(node_ref.left.clone());
                }
                if node_ref.right.is_some() {
                    stack.push(node_ref.right.clone());
                }
            }
            acc as i64
        };
        let mut result = 0;
        dfs(root, sum, &mut result);
        (result % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let root = TreeNode::from(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        let result = Solution::max_product(root);
        assert_eq!(110, result)
    }

    #[test]
    fn test_case_02() {
        let root = TreeNode::from(vec![
            Some(1),
            None,
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            Some(5),
            Some(6),
        ]);
        let result = Solution::max_product(root);
        assert_eq!(90, result)
    }
}
