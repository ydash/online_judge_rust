// 502. IPO

use std::collections::BinaryHeap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut pq = BinaryHeap::new();
        let mut indices = (0..n).collect::<Vec<_>>();
        indices.sort_by(|&a, &b| capital[b].cmp(&capital[a]));
        let mut current_capital = w;
        for _ in 0..k {
            while indices
                .last()
                .map_or(false, |&i| capital[i] <= current_capital)
            {
                let i = indices.pop().unwrap();
                pq.push(profits[i]);
            }
            pq.pop().map(|p| current_capital += p);
        }
        current_capital
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]);
        assert_eq!(6, result)
    }
}
