use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::util::TreeNode;

// 102. Binary Tree Level Order Traversal
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut result = vec![];
        let mut q = VecDeque::from([root.unwrap()]);
        while !q.is_empty() {
            let mut tmp = vec![];
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let node = node.borrow();
                tmp.push(node.val);
                if let Some(l) = node.left.clone() {
                    q.push_back(Rc::clone(&l));
                }
                if let Some(r) = node.right.clone() {
                    q.push_back(Rc::clone(&r));
                }
            }
            result.push(tmp);
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
        let result = Solution::level_order(Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        }))));
        assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], result)
    }
    #[test]
    fn test_case_02() {
        let result = Solution::level_order(Some(Rc::new(RefCell::new(TreeNode::new(1)))));
        assert_eq!(vec![vec![1]], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::level_order(None);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(expected, result)
    }
}
