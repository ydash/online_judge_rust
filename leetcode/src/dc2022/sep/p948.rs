// 948. Bag of Tokens

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        tokens.sort();
        let mut left = 0;
        let mut right = tokens.len() - 1;
        let mut score = 0;
        let mut rest_power = power;
        let mut result = 0;
        while left <= right {
            if rest_power >= tokens[left] {
                rest_power -= tokens[left];
                score += 1;
                left += 1;
                result = result.max(score);
            } else if score > 0 {
                rest_power += tokens[right];
                right -= 1;
                score -= 1;
            } else {
                break;
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
        let result = Solution::bag_of_tokens_score(vec![100], 50);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::bag_of_tokens_score(vec![100, 200], 150);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200);
        assert_eq!(2, result)
    }
}
