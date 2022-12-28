// 1962. Remove Stones to Minimize the Total

use std::collections::BinaryHeap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut pq = BinaryHeap::from(piles);
        for _ in 0..k {
            pq.pop().map(|v| {
                pq.push(v - v / 2);
            });
        }
        pq.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_stone_sum(vec![5, 4, 9], 2);
        assert_eq!(12, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_stone_sum(vec![4, 3, 6, 7], 3);
        assert_eq!(12, result)
    }
}
