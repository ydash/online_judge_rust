// 2279. Maximum Bags With Full Capacity of Rocks

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let n = capacity.len();
        let mut diff: Vec<i32> = (0..n).map(|i| capacity[i] - rocks[i]).collect();
        diff.sort_unstable();
        let mut result = 0;
        let mut rest = additional_rocks;
        for d in diff.into_iter() {
            if d <= rest {
                rest -= d;
                result += 1;
            } else {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::maximum_bags(vec![10, 2, 2], vec![2, 2, 0], 100);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::maximum_bags(vec![10], vec![1], 1);
        assert_eq!(0, result)
    }
}
