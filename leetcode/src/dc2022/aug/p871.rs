// 871. Minimum Number of Refueling Stops

use std::collections::BinaryHeap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut pq = BinaryHeap::new();
        let mut total = start_fuel;
        let mut i = 0;
        while total < target {
            while i < stations.len() && stations[i][0] <= total {
                pq.push(stations[i][1]);
                i += 1;
            }
            if let Some(fuel) = pq.pop() {
                total += fuel;
                count += 1;
            } else {
                return -1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_refuel_stops(1, 1, vec![]);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_refuel_stops(100, 1, vec![vec![10, 100]]);
        assert_eq!(-1, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_refuel_stops(
            100,
            10,
            vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]],
        );
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::min_refuel_stops(100, 50, vec![vec![25, 25], vec![50, 50]]);
        assert_eq!(1, result)
    }
}
