// 45. Jump Game II

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut jump_lim = 0;
        let mut next_lim = 0;
        let mut result = 0;
        for i in 0..n - 1 {
            next_lim = next_lim.max(i + nums[i] as usize);
            if i == jump_lim {
                result += 1;
                jump_lim = next_lim;
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
        let result = Solution::jump(vec![2, 3, 1, 1, 4]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::jump(vec![2, 3, 0, 1, 4]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::jump(vec![0]);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::jump(vec![2, 1]);
        assert_eq!(1, result)
    }
}
