// 958. Check Completeness of a Binary Tree

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();
        let mut exist_prev = true;
        queue.push_back(root);
        while !queue.is_empty() {
            if let Some(x) = queue.pop_front().flatten() {
                if !exist_prev {
                    return false;
                }
                let x = x.borrow();
                queue.push_back(x.left.clone());
                queue.push_back(x.right.clone());
            } else {
                exist_prev = false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::is_complete_tree(TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
        ]));
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_complete_tree(TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(7),
        ]));
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::is_complete_tree(TreeNode::from(vec![Some(1)]));
        assert_eq!(true, result)
    }
}
