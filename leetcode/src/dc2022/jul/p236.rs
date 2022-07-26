// 236. Lowest Common Ancestor of a Binary Tree

use std::cell::RefCell;
use std::rc::Rc;

use crate::util::TreeNode;

const BOTH_PENDING: i32 = 0;
const LEFT_DONE: i32 = 1;
const BOTH_DONE: i32 = 2;
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![];
        let mut lca = None;
        let mut one_node_found = false;
        stack.push((BOTH_PENDING, root.unwrap()));
        while !stack.is_empty() {
            let elem = stack.last().unwrap();
            let state = elem.0;
            let node = Rc::clone(&elem.1);
            if state != BOTH_DONE {
                if state == BOTH_PENDING {
                    if node == p.clone().unwrap() || node == q.clone().unwrap() {
                        if one_node_found {
                            return lca;
                        } else {
                            one_node_found = true;
                            lca = stack.last().map(|v| Rc::clone(&v.1));
                        }
                    }
                    stack.pop();
                    stack.push((LEFT_DONE, Rc::clone(&node)));
                    if let Some(left) = node.borrow().left.clone() {
                        stack.push((BOTH_PENDING, left));
                    }
                } else {
                    stack.pop();
                    stack.push((BOTH_DONE, Rc::clone(&node)));
                    if let Some(right) = node.borrow().right.clone() {
                        stack.push((BOTH_PENDING, right));
                    }
                }
            } else {
                stack.pop();
                if one_node_found && lca.clone().unwrap() == node {
                    lca = stack.last().map(|v| Rc::clone(&v.1));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let q = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: q.clone(),
            }))),
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: p.clone(),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
            }))),
        })));
        let result = Solution::lowest_common_ancestor(root, p.clone(), q);
        assert_eq!(p.clone(), result)
    }

    #[test]
    fn test_case_02() {
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
        })));
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: p.clone(),
            right: q.clone(),
        })));
        let result = Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone());
        assert_eq!(root.clone(), result)
    }
}
