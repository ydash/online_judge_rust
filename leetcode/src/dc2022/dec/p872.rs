// 872. Leaf-Similar Trees

use std::{cell::RefCell, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, acc: &mut Vec<i32>) {
            if let Some(x) = node {
                let x = x.borrow();
                if x.left.is_none() && x.right.is_none() {
                    acc.push(x.val);
                } else {
                    dfs(x.left.clone(), acc);
                    dfs(x.right.clone(), acc);
                }
            }
        }
        let mut leaves1 = vec![];
        let mut leaves2 = vec![];
        dfs(root1, &mut leaves1);
        dfs(root2, &mut leaves2);
        leaves1 == leaves2
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
            }))),
        })));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                }))),
            }))),
        })));
        let result = Solution::leaf_similar(root1, root2);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));
        let result = Solution::leaf_similar(root1, root2);
        assert_eq!(false, result)
    }
}
