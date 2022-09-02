// 637. Average of Levels in Binary Tree

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut queue = VecDeque::new();
        let mut result = vec![];
        queue.push_back(root);
        while !queue.is_empty() {
            let num = queue.len() as f64;
            let mut sum = 0_f64;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                if let Some(rc) = node {
                    let node = rc.borrow();
                    sum += node.val as f64;
                    if node.left.is_some() {
                        queue.push_back(node.left.clone());
                    }
                    if node.right.is_some() {
                        queue.push_back(node.right.clone());
                    }
                }
            }
            result.push(sum / num);
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
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        let result = Solution::average_of_levels(root);
        assert_eq!(vec![3_f64, 14.5, 11_f64], result)
    }
}
