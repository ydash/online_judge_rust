// 2389. Longest Subsequence With Limited Sum

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort();
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        queries
            .iter()
            .map(|q| match nums.binary_search(q) {
                Ok(x) => x + 1,
                Err(x) => x,
            } as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]);
        assert_eq!(vec![2, 3, 4], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::answer_queries(vec![2, 3, 4, 5], vec![1]);
        assert_eq!(vec![0], result)
    }
}
