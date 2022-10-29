// 2136. Earliest Possible Day of Full Bloom

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let n = plant_time.len();
        let mut indices = (0..n).collect::<Vec<_>>();
        indices.sort_by(|&i, &j| grow_time[j].cmp(&grow_time[i]));
        indices
            .into_iter()
            .fold((0, 0), |(current, result), i| {
                let planted_time = current + plant_time[i];
                (planted_time, result.max(planted_time + grow_time[i]))
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1]);
        assert_eq!(9, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::earliest_full_bloom(vec![1, 2, 3, 2], vec![2, 1, 2, 1]);
        assert_eq!(9, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::earliest_full_bloom(vec![1], vec![1]);
        assert_eq!(2, result)
    }
}
