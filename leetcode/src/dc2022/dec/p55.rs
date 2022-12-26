// 55. Jump Game

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_idx = 0;
        for i in 0..nums.len() {
            if max_idx < i {
                return false;
            }
            max_idx = max_idx.max(i + nums[i] as usize);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::can_jump(vec![2, 3, 1, 1, 4]);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::can_jump(vec![3, 2, 1, 0, 4]);
        assert_eq!(false, result)
    }
}
