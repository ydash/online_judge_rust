// 983. Minimum Cost For Tickets

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = [0; 366];
        let mut current = 1;
        for d in days.iter().map(|&d| d as usize) {
            while current < d {
                dp[current] = dp[current - 1];
                current += 1;
            }
            dp[current] = (dp[current - 1] + costs[0])
                .min(dp[0.max(current.max(7) - 7)] + costs[1])
                .min(dp[current.max(30) - 30] + costs[2]);
            current += 1;
        }
        dp[days[days.len() - 1] as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]);
        assert_eq!(11, result)
    }

    #[test]
    fn test_case_02() {
        let result =
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]);
        assert_eq!(17, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![7, 2, 15]);
        assert_eq!(6, result)
    }
}
