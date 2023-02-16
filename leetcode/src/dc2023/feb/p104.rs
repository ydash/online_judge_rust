// 104. Maximum Depth of Binary Tree

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut q = VecDeque::new();
        q.push_back(root.clone());
        let mut depth = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let x = q.pop_front().unwrap().unwrap();
                let x = x.borrow();
                if x.left.is_some() {
                    q.push_back(x.left.clone());
                }
                if x.right.is_some() {
                    q.push_back(x.right.clone());
                }
            }
            depth += 1;
        }
        depth
    }
}

#[cfg(test)]
mod tests {
    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::max_depth(TreeNode::from(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]));
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::max_depth(TreeNode::from(vec![Some(1)]));
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::max_depth(None);
        assert_eq!(0, result)
    }
}
