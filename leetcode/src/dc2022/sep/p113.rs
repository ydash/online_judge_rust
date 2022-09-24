// 113. Path Sum II

use std::cell::RefCell;
use std::rc::Rc;

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn dfs(
            current: Option<Rc<RefCell<TreeNode>>>,
            sum: i32,
            path: &mut Vec<i32>,
            target: i32,
            result: &mut Vec<Vec<i32>>,
        ) {
            if let Some(x) = current.clone() {
                let x = x.borrow();
                let v = x.val;
                let sum = v + sum;
                path.push(v);
                if x.left.is_none() && x.right.is_none() {
                    if sum == target {
                        result.push(path.clone());
                    }
                } else {
                    dfs(x.left.clone(), sum, path, target, result);
                    dfs(x.right.clone(), sum, path, target, result);
                }
                path.pop();
            }
        }
        let mut result = vec![];
        dfs(root, 0, &mut vec![], target_sum, &mut result);
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
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
            }))),
        })));
        let result = Solution::path_sum(root, 22);
        assert_eq!(vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::path_sum(None, 0);
        assert_eq!(Vec::<Vec<i32>>::new(), result)
    }

    #[test]
    fn test_case_03() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));
        let result = Solution::path_sum(root, 1);
        assert_eq!(Vec::<Vec<i32>>::new(), result)
    }
}
