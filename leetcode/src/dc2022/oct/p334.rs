// 334. Increasing Triplet Subsequence

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = None;
        let mut second = None;
        for n in nums.into_iter() {
            if second.is_some() && second.unwrap() < n {
                return true;
            } else if first.is_some() && first.unwrap() < n {
                second = Some(n);
            } else {
                first = Some(n);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::increasing_triplet(vec![1, 2, 3, 4, 5]);
        assert!(result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::increasing_triplet(vec![5, 4, 3, 2, 1]);
        assert!(!result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]);
        assert!(result)
    }
}
