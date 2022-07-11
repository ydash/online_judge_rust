use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// 199. Binary Tree Right Side View
#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut buf = vec![];
        let mut q = VecDeque::new();
        q.push_back(root.unwrap());
        while !q.is_empty() {
            let right_end = Rc::clone(q.back().unwrap());
            let right_end = right_end.borrow();
            buf.push(right_end.val);

            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let node = node.borrow();
                if let Some(l) = node.left.clone() {
                    q.push_back(l);
                }
                if let Some(r) = node.right.clone() {
                    q.push_back(r);
                }
            }
        }
        buf
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::{Solution, TreeNode};

    #[test]
    fn test_case_01() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));
        let result = Solution::right_side_view(root);
        assert_eq!(vec![1, 3, 4], result)
    }

    #[test]
    fn test_case_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let result = Solution::right_side_view(root);
        assert_eq!(result, vec![1, 3])
    }

    #[test]
    fn test_case_03() {
        let root = None;
        let result = Solution::right_side_view(root);
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, result)
    }

    #[test]
    fn test_case_04() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        })));
        let result = Solution::right_side_view(root);
        assert_eq!(result, vec![1, 2])
    }

    #[test]
    fn test_case_05() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let result = Solution::right_side_view(root);
        assert_eq!(result, vec![1, 3, 4])
    }
}
