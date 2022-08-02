// 378. Kth Smallest Element in a Sorted Matrix

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let k = k as usize;
        let mut pq = BinaryHeap::new();
        for i in 0..k.min(m) {
            pq.push(Reverse(Item {
                val: matrix[i][0],
                row: i,
                col: 0,
            }));
        }
        for _ in 1..k {
            let Item { val: _, row, col } = pq.pop().unwrap().0;
            if col + 1 < n {
                pq.push(Reverse(Item {
                    val: matrix[row][col + 1],
                    row,
                    col: col + 1,
                }));
            }
        }
        pq.pop().unwrap().0.val
    }
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
struct Item {
    val: i32,
    row: usize,
    col: usize,
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result =
            Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8);
        assert_eq!(13, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::kth_smallest(vec![vec![-5]], 1);
        assert_eq!(-5, result)
    }
}
