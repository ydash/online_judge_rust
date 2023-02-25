// 121. Best Time to Buy and Sell Stock

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = prices[0];
        let mut result = 0;
        for price in prices.into_iter().skip(1) {
            result = result.max(price - min_price);
            min_price = min_price.min(price)
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::max_profit(vec![7, 6, 4, 3, 1]);
        assert_eq!(0, result)
    }
}
