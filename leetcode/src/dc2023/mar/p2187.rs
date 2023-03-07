// 2187. Minimum Time to Complete Trips

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        fn greater_than_or_equal_to_total_trips(
            time: &[i32],
            total_time: i64,
            total_trips: i64,
        ) -> bool {
            let mut count = 0;
            for &t in time {
                count += total_time / (t as i64);
                if total_trips <= count {
                    return true;
                }
            }
            total_trips <= count
        }
        let total_trips = total_trips as i64;
        let mut left: i64 = 0;
        let mut right: i64 = time[0] as i64 * total_trips as i64;
        while left < right {
            let mid = left + (right - left) / 2;
            if greater_than_or_equal_to_total_trips(&time, mid, total_trips) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::minimum_time(vec![1, 2, 3], 5);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::minimum_time(vec![2], 1);
        assert_eq!(2, result)
    }
}
