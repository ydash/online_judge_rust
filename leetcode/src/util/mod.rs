use std::{cell::RefCell, rc::Rc};

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
