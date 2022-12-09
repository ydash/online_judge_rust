use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

    pub fn from(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        let mut nums = VecDeque::from(nums);
        if let Some(root_v) = nums.pop_front().and_then(|it| it) {
            let mut queue = VecDeque::new();
            let root = Some(Rc::new(RefCell::new(Self::new(root_v))));
            queue.push_back(root.clone());
            while !nums.is_empty() {
                let node = queue.pop_front().and_then(|n| n).unwrap();
                let mut node_ref = node.borrow_mut();
                nums.pop_front().and_then(|l| l).map(|lv| {
                    let left = Some(Rc::new(RefCell::new(TreeNode::new(lv))));
                    node_ref.left = left.clone();
                    queue.push_back(left);
                });
                nums.pop_front().and_then(|r| r).map(|rv| {
                    let right = Some(Rc::new(RefCell::new(TreeNode::new(rv))));
                    node_ref.right = right.clone();
                    queue.push_back(right);
                });
            }
            root
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from(nums: Vec<i32>) -> Option<Box<Self>> {
        let mut current = None;
        for i in (0..nums.len()).rev() {
            current = Some(Box::new(ListNode {
                val: nums[i],
                next: current,
            }))
        }
        current
    }
}

pub fn str_vec_2_string_vec(slice: Vec<&str>) -> Vec<String> {
    slice.into_iter().map(|s| s.into()).collect()
}
