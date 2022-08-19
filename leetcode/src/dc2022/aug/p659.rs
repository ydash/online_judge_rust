// 659. Split Array into Consecutive Subsequences

use std::{cmp::Ordering, collections::BinaryHeap};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut pq: BinaryHeap<Elem> = BinaryHeap::new();
        pq.push(Elem { seq: vec![nums[0]] });
        for &n in nums.iter().skip(1) {
            while !pq.is_empty() && pq.peek().and_then(|elm| elm.seq.last()).unwrap() + 1 < n {
                let elm = pq.pop().unwrap();
                if elm.seq.len() < 3 {
                    return false;
                }
            }
            if !pq.is_empty() && pq.peek().and_then(|e| e.seq.last()).unwrap() + 1 == n {
                let mut elm = pq.pop().unwrap();
                elm.seq.push(n);
                pq.push(elm);
            } else {
                pq.push(Elem { seq: vec![n] });
            }
        }
        pq.iter().all(|elm| elm.seq.len() >= 3)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Elem {
    seq: Vec<i32>,
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        let v1 = self.seq.last().unwrap();
        let v2 = other.seq.last().unwrap();
        match v2.cmp(v1) {
            Ordering::Equal => other.seq.len().cmp(&self.seq.len()),
            other => other,
        }
    }
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::is_possible(vec![1, 2, 3, 3, 4, 5]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::is_possible(vec![1, 2, 3, 4, 4, 5]);
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::is_possible(vec![1, 2, 3, 5, 6, 7]);
        assert_eq!(true, result)
    }
}
