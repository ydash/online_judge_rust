// 1383. Maximum Performance of a Team

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut result: i64 = 0;
        let mut pq = BinaryHeap::new();
        let mut total_speed: i64 = 0;
        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by(|&a, &b| efficiency[b].cmp(&efficiency[a]));
        for (count, i) in indices.into_iter().enumerate() {
            let spd = speed[i] as i64;
            let eff = efficiency[i] as i64;
            pq.push(Reverse(spd));
            total_speed += spd;
            if count < k {
                result = result.max(total_speed * eff);
            } else {
                total_speed -= pq.pop().unwrap().0;
                result = result.max(total_speed * eff);
            }
        }
        (result % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_case_01() {
        let result =
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2);
        assert_eq!(result, 60)
    }

    #[test]
    fn test_case_02() {
        let result =
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3);
        assert_eq!(result, 68)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::max_performance(3, vec![2, 8, 2], vec![2, 7, 1], 2);
        assert_eq!(56, result)
    }
}
