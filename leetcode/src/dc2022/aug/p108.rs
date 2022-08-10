// 108. Convert Sorted Array to Binary Search Tree

use std::cell::RefCell;
use std::rc::Rc;

use crate::util::TreeNode;
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                None
            } else {
                let mid = nums.len() / 2;
                let node = TreeNode {
                    val: nums[mid],
                    left: build_tree(&nums[..mid]),
                    right: build_tree(&nums[(mid + 1)..]),
                };
                Some(Rc::new(RefCell::new(node)))
            }
        }
        build_tree(&nums[..])
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::sorted_array_to_bst(vec![1]);
        let expected = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(expected, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::sorted_array_to_bst(vec![1, 3]);
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: None,
        })));
        assert_eq!(expected, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(-10)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: None,
            }))),
        })));
        assert_eq!(expected, result)
    }
}
