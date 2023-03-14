// 129. Sum Root to Leaf Numbers

use std::{cell::RefCell, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, acc: i32, result: &mut i32) {
            if let Some(x) = node {
                let x = x.borrow();
                let acc = acc * 10 + x.val;
                if x.left.is_none() && x.right.is_none() {
                    *result += acc;
                } else {
                    dfs(x.left.clone(), acc, result);
                    dfs(x.right.clone(), acc, result);
                }
            }
        }
        let mut result = 0;
        dfs(root, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::sum_numbers(TreeNode::from(vec![Some(1), Some(2), Some(3)]));
        assert_eq!(25, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::sum_numbers(TreeNode::from(vec![
            Some(4),
            Some(9),
            Some(0),
            Some(5),
            Some(1),
        ]));
        assert_eq!(1026, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::sum_numbers(TreeNode::from(vec![Some(3)]));
        assert_eq!(3, result)
    }
}
