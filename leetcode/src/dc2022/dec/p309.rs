// 309. Best Time to Buy and Sell Stock with Cooldown

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut sell = vec![0; n + 1];
        let mut buy = vec![0; n + 1];
        buy[1] = -prices[0];
        for i in 1..n {
            sell[i + 1] = sell[i].max(buy[i] + prices[i]);
            buy[i + 1] = buy[i].max(sell[i - 1] - prices[i]);
        }
        sell[n]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::max_profit(vec![1, 2, 3, 0, 2]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::max_profit(vec![1]);
        assert_eq!(0, result)
    }
}
