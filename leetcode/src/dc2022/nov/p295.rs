// 295. Find Median from Data Stream

use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}
#[allow(dead_code)]
impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.left.is_empty() || *self.left.peek().unwrap() >= num {
            self.left.push(num);
        } else {
            self.right.push(Reverse(num));
        }

        if self.left.len() > self.right.len() + 1 {
            self.left.pop().map(|n| self.right.push(Reverse(n)));
        } else if self.right.len() > self.left.len() + 1 {
            self.right.pop().map(|n| self.left.push(n.0));
        }
    }

    fn find_median(&self) -> f64 {
        if self.left.len() > self.right.len() {
            *self.left.peek().unwrap() as f64
        } else if self.right.len() > self.left.len() {
            self.right.peek().unwrap().0 as f64
        } else {
            (self.left.peek().unwrap() + self.right.peek().unwrap().0) as f64 / 2_f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MedianFinder;

    #[test]
    fn test_case_01() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        assert_eq!(1_f64, mf.find_median());
        mf.add_num(2);
        assert_eq!(1.5, mf.find_median());
        mf.add_num(3);
        assert_eq!(2_f64, mf.find_median());
    }
}
