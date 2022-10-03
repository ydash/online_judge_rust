// 1578. Minimum Time to Make Rope Colorful

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors = colors.as_bytes();
        let mut prev = 0;
        let mut result = 0;
        for i in 1..colors.len() {
            if colors[prev] == colors[i] {
                if needed_time[prev] < needed_time[i] {
                    result += needed_time[prev];
                    prev = i;
                } else {
                    result += needed_time[i];
                }
            } else {
                prev = i;
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
        let result = Solution::min_cost("abaac".to_owned(), vec![1, 2, 3, 4, 5]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_cost("abc".to_owned(), vec![1, 2, 3]);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_cost("aabaa".to_owned(), vec![1, 2, 3, 4, 1]);
        assert_eq!(2, result)
    }
}
