// 1833. Maximum Ice Cream Bars

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_ice_cream(mut costs: Vec<i32>, coins: i32) -> i32 {
        costs.sort();
        let mut rest = coins;
        let mut result = 0;
        for c in costs.into_iter() {
            if c > rest {
                break;
            }
            rest -= c;
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20);
        assert_eq!(6, result)
    }
}
