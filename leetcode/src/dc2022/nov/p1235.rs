// 1235. Maximum Profit in Job Scheduling

use std::collections::BTreeMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = profit.len();
        let mut indices = (0..n).collect::<Vec<_>>();
        indices.sort_by(|&i, &j| end_time[i].cmp(&end_time[j]));
        let mut dp: BTreeMap<i32, i32> = BTreeMap::new();
        for &i in indices.iter() {
            let st = start_time[i];
            let et = end_time[i];
            let sum_profit = profit[i] + dp.range(..=st).last().map_or(0, |p| *p.1);
            let prev_max = dp.range(..=et).last().map_or(0, |p| *p.1);
            dp.insert(et, prev_max.max(sum_profit));
        }
        *dp.get(&end_time[indices[n - 1]]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result =
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]);
        assert_eq!(120, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::job_scheduling(
            vec![1, 2, 3, 4, 6],
            vec![3, 5, 10, 6, 9],
            vec![20, 20, 100, 70, 60],
        );
        assert_eq!(150, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::job_scheduling(
            vec![1, 2, 3, 4, 6],
            vec![3, 5, 10, 6, 9],
            vec![20, 100, 100, 70, 60],
        );
        assert_eq!(160, result)
    }
}
