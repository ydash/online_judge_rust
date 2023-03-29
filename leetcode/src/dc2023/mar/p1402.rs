// 1402. Reducing Dishes

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        let mut d = 0;
        let mut result = 0;
        while let Some(x) = satisfaction.pop() {
            d += x;
            if d <= 0 {
                break;
            }
            result += d;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]);
        assert_eq!(14, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::max_satisfaction(vec![4, 3, 2]);
        assert_eq!(20, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::max_satisfaction(vec![-1, -4, -5]);
        assert_eq!(0, result)
    }
}
