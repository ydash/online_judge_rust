// 57. Insert Interval

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let ps = intervals.partition_point(|v| v[1] < new_interval[0]);
        let pe = intervals.partition_point(|v| v[0] <= new_interval[1]);
        if ps == pe {
            intervals.insert(ps, new_interval);
        } else {
            let new_interval = vec![
                intervals[ps][0].min(new_interval[0]),
                intervals[pe - 1][1].max(new_interval[1]),
            ];
            intervals.drain(ps..pe);
            intervals.insert(ps, new_interval);
        }
        intervals
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]);
        assert_eq!(vec![vec![1, 5], vec![6, 9]], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::insert(vec![vec![1, 2]], vec![0, 3]);
        assert_eq!(vec![vec![0, 3]], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::insert(vec![vec![1, 2]], vec![3, 4]);
        assert_eq!(vec![vec![1, 2], vec![3, 4]], result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::insert(vec![vec![2, 3]], vec![0, 1]);
        assert_eq!(vec![vec![0, 1], vec![2, 3]], result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::insert(
            vec![
                vec![1, 2],
                vec![3, 4],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16],
            ],
            vec![4, 8],
        );
        assert_eq!(vec![vec![1, 2], vec![3, 10], vec![12, 16]], result)
    }

    #[test]
    fn test_case_06() {
        let result = Solution::insert(vec![], vec![0, 1]);
        assert_eq!(vec![vec![0, 1]], result)
    }
}
