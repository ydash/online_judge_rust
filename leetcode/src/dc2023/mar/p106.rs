// 106. Construct Binary Tree from Inorder and Postorder Traversal

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn aux(
            inorder: &[i32],
            postorder: &[i32],
            indices: &HashMap<i32, usize>,
            d: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let n = postorder.len();
            if n == 0 {
                None
            } else {
                let val = postorder[n - 1];
                let i = *indices.get(&val).unwrap() - d;
                Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: aux(&inorder[..i], &postorder[..i], indices, d),
                    right: aux(&inorder[i + 1..], &postorder[i..n - 1], indices, d + i + 1),
                })))
            }
        }
        let mut indices = HashMap::new();
        for (i, &v) in inorder.iter().enumerate() {
            indices.insert(v, i);
        }
        aux(&inorder, &postorder, &indices, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::TreeNode;

    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]);
        assert_eq!(
            TreeNode::from(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ]),
            result
        )
    }

    #[test]
    fn test_case_02() {
        let result = Solution::build_tree(vec![-1], vec![-1]);
        assert_eq!(TreeNode::from(vec![Some(-1)]), result)
    }
}
