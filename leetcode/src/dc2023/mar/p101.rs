// 101. Symmetric Tree

use std::{cell::RefCell, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack1 = vec![];
        let mut stack2 = vec![];
        if let Some(x) = root {
            let x = x.borrow();
            stack1.push(x.left.clone());
            stack2.push(x.right.clone());
        }
        while !stack1.is_empty() || !stack2.is_empty() {
            match (stack1.pop().flatten(), stack2.pop().flatten()) {
                (None, None) => (),
                (Some(a), Some(b)) => {
                    let a = a.borrow();
                    let b = b.borrow();
                    if a.val != b.val {
                        return false;
                    }
                    stack1.push(a.right.clone());
                    stack1.push(a.left.clone());
                    stack2.push(b.left.clone());
                    stack2.push(b.right.clone());
                }
                _ => return false,
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
        let result = Solution::is_symmetric(TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ]));
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_symmetric(TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(2),
            None,
            Some(3),
            None,
            Some(3),
        ]));
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::is_symmetric(TreeNode::from(vec![Some(1)]));
        assert_eq!(true, result)
    }
}
