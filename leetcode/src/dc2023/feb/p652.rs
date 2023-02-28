// 652. Find Duplicate Subtrees

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            seen_twice: &mut HashMap<String, bool>,
            result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> String {
            match root {
                Some(rc_of_node) => {
                    let ref_of_node = rc_of_node.borrow();
                    let s = format!(
                        "({}{}{})",
                        dfs(ref_of_node.left.clone(), seen_twice, result),
                        ref_of_node.val,
                        dfs(ref_of_node.right.clone(), seen_twice, result)
                    );
                    if let Some(is_seen) = seen_twice.get_mut(&s) {
                        if !*is_seen {
                            result.push(Some(rc_of_node.clone()));
                            *is_seen = true;
                        }
                    } else {
                        seen_twice.insert(s.clone(), false);
                    }
                    s
                }
                None => String::new(),
            }
        }
        let mut seen_twice = HashMap::new();
        let mut result = vec![];
        dfs(root, &mut seen_twice, &mut result);
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
        let result = Solution::find_duplicate_subtrees(TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            Some(2),
            Some(4),
            None,
            None,
            Some(4),
        ]));
        assert_eq!(
            vec![
                TreeNode::from(vec![Some(4)]),
                TreeNode::from(vec![Some(2), Some(4)])
            ],
            result
        )
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_duplicate_subtrees(TreeNode::from(vec![Some(1)]));
        assert_eq!(Vec::<Option<Rc<RefCell<TreeNode>>>>::new(), result)
    }
}
