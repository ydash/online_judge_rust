// 987. Vertical Order Traversal of a Binary Tree

use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::rc::Rc;

use crate::util::TreeNode;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut col: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((root, 0));
        let mut count = 0;
        while !queue.is_empty() {
            let mut tmp: HashMap<i32, BinaryHeap<Reverse<i32>>> = HashMap::new();
            for _ in 0..queue.len() {
                if let (Some(node), c) = queue.pop_front().unwrap() {
                    let node = node.borrow();
                    let pq = tmp.entry(c).or_insert(BinaryHeap::new());
                    pq.push(Reverse(node.val));
                    if node.left.is_some() {
                        queue.push_back((node.left.clone(), c - 1));
                    }
                    if node.right.is_some() {
                        queue.push_back((node.right.clone(), c + 1));
                    }
                }
            }
            for i in -count..=count {
                if let Some(mut pq) = tmp.remove(&i) {
                    let buf = col.entry(i).or_insert(vec![]);
                    while !pq.is_empty() {
                        buf.push(pq.pop().unwrap().0);
                    }
                }
            }
            count += 1;
        }
        let mut result = vec![];
        for i in -(count - 1)..count {
            if let Some(vc) = col.remove(&i) {
                result.push(vc);
            }
        }
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
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        let result = Solution::vertical_traversal(root);
        assert_eq!(vec![vec![9], vec![3, 15], vec![20], vec![7]], result)
    }

    #[test]
    fn test_case_02() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        let result = Solution::vertical_traversal(root);
        assert_eq!(
            vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]],
            result
        )
    }
}
