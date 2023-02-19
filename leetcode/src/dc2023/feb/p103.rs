// 103. Binary Tree Zigzag Level Order Traversal

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut q = VecDeque::new();
        if root.is_some() {
            q.push_back(root.clone());
        }
        let mut reverse = false;
        while !q.is_empty() {
            let mut same_level = VecDeque::new();
            for _ in 0..q.len() {
                let x = q.pop_front().flatten().unwrap();
                let x = x.borrow();
                if reverse {
                    same_level.push_front(x.val);
                } else {
                    same_level.push_back(x.val);
                }
                if let Some(l) = x.left.clone() {
                    q.push_back(Some(l));
                }
                if let Some(r) = x.right.clone() {
                    q.push_back(Some(r));
                }
            }
            result.push(same_level.into_iter().collect::<Vec<_>>());
            reverse = !reverse;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::zigzag_level_order(TreeNode::from(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]));
        assert_eq!(vec![vec![3], vec![20, 9], vec![15, 7]], result);
    }

    #[test]
    fn test_case_02() {
        let result = Solution::zigzag_level_order(TreeNode::from(vec![Some(1)]));
        assert_eq!(vec![vec![1]], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::zigzag_level_order(TreeNode::from(vec![None]));
        assert_eq!(Vec::<Vec<i32>>::new(), result)
    }
}
