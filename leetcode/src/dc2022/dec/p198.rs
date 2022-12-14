// 198. House Robber

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut a = 0;
        let mut b = 0;
        for i in 0..n {
            let tmp = b.max(a + nums[i]);
            a = b;
            b = tmp
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::rob(vec![1, 2, 3, 1]);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::rob(vec![2, 1]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::rob(vec![42]);
        assert_eq!(42, result)
    }
}
