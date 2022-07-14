use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::util::TreeNode;

// 105. Construct Binary Tree from Preorder and Inorder Traversal
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_indices: HashMap<_, _> =
            inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        Self::array_to_tree(&preorder, &inorder_indices, 0)
    }

    fn array_to_tree(
        preorder: &[i32],
        inorder_indices: &HashMap<i32, usize>,
        inorder_left_bound: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(&val) = preorder.first() {
            let mut root = TreeNode::new(val);
            let inorder_index = inorder_indices[&val];
            let left_node = Self::array_to_tree(
                &preorder[1..=(inorder_index - inorder_left_bound)],
                inorder_indices,
                inorder_left_bound,
            );
            let right_node = Self::array_to_tree(
                &preorder[(inorder_index - inorder_left_bound + 1)..],
                inorder_indices,
                inorder_index + 1,
            );
            root.left = left_node;
            root.right = right_node;
            Some(Rc::new(RefCell::new(root)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        assert_eq!(expected, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::build_tree(vec![3, 9, 7, 5, 8], vec![7, 9, 5, 3, 8]);
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
        })));
        assert_eq!(expected, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::build_tree(vec![3, 20, 15, 7], vec![3, 15, 20, 7]);
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        assert_eq!(expected, result)
    }
}
