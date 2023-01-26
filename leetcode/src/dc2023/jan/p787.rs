// 787. Cheapest Flights Within K Stops

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let dst = dst as usize;
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for v in flights {
            adj[v[0] as usize].push((v[1] as usize, v[2]));
        }
        let mut min_stop = vec![k; n];
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, src as usize, -1)));
        while let Some(Reverse((cost, current, stop))) = pq.pop() {
            if current == dst {
                return cost;
            }
            if stop < min_stop[current] {
                min_stop[current] = stop;
                for &(next, c) in adj[current].iter() {
                    pq.push(Reverse((cost + c, next, stop + 1)));
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::find_cheapest_price(
            4,
            vec![
                vec![0, 1, 100],
                vec![1, 2, 100],
                vec![2, 0, 100],
                vec![1, 3, 600],
                vec![2, 3, 200],
            ],
            0,
            3,
            1,
        );
        assert_eq!(700, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_cheapest_price(
            4,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            1,
        );
        assert_eq!(200, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::find_cheapest_price(
            4,
            vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
            0,
            2,
            0,
        );
        assert_eq!(500, result)
    }
}
