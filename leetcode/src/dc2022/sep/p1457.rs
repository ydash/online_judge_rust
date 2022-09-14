// 1457. Pseudo-Palindromic Paths in a Binary Tree

use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = vec![0; 10];
        fn dfs(node: Ref<TreeNode>, count: &mut Vec<i32>) -> i32 {
            let v = node.val as usize;
            count[v] += 1;
            let mut result = 0;
            let left = node.left.clone();
            let right = node.right.clone();
            if left.is_none() && right.is_none() {
                result = if count.iter().filter(|&x| x % 2 == 1).count() <= 1 {
                    1
                } else {
                    0
                }
            } else {
                if left.is_some() {
                    result += dfs(left.unwrap().borrow(), count);
                }
                if right.is_some() {
                    result += dfs(right.unwrap().borrow(), count);
                }
            }
            count[v] -= 1;
            result
        }
        dfs(root.unwrap().borrow(), &mut count)
    }
}
