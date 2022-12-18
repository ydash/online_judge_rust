// 739. Daily Temperatures

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut result = vec![0; n];
        let mut stack = vec![];
        for i in 0..n {
            while stack
                .last()
                .map_or(false, |&j| temperatures[j] < temperatures[i])
            {
                stack.pop().map(|j| {
                    result[j] = (i - j) as i32;
                });
            }
            stack.push(i);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
        assert_eq!(vec![1, 1, 4, 2, 1, 1, 0, 0], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::daily_temperatures(vec![30, 40, 50, 60]);
        assert_eq!(vec![1, 1, 1, 0], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::daily_temperatures(vec![70, 40, 35, 36, 50]);
        assert_eq!(vec![0, 3, 1, 1, 0], result)
    }
}
