#[cfg(test)]
mod tests {
    use super::solution::max_result;

    #[test]
    fn test_case_01() {
        let result = max_result(vec![1, -1, -2, 4, -7, 3], 2);
        assert_eq!(7, result)
    }

    #[test]
    fn test_case_02() {
        let result = max_result(vec![10, -5, -2, 4, 0, 3], 3);
        assert_eq!(17, result)
    }

    #[test]
    fn test_case_03() {
        let result = max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_04() {
        let result = max_result(vec![8, 34], 1);
        assert_eq!(42, result)
    }
}

// 1696. Jump Game VI
mod solution {
    use std::collections::VecDeque;

    #[allow(dead_code)]
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![0; nums.len()];
        let mut deq = VecDeque::new();
        dp[0] = nums[0];
        deq.push_back(0);
        for i in 1..nums.len() {
            if i > k && deq.front().map(|&j| j < i - k).unwrap_or(false) {
                deq.pop_front();
            }
            dp[i] = dp[*deq.front().unwrap()] + nums[i];
            while !deq.is_empty() && dp[*deq.back().unwrap()] <= dp[i] {
                deq.pop_back();
            }
            deq.push_back(i);
        }
        dp[nums.len() - 1]
    }
}
