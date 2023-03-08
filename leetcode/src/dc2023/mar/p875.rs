// 875. Koko Eating Bananas

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        while left < right {
            let mid = left + (right - left) / 2;
            if piles.iter().fold(0, |acc, &p| {
                acc + p / mid + if p % mid == 0 { 0 } else { 1 }
            }) <= h
            {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_eating_speed(vec![3, 6, 7, 11], 8);
        assert_eq!(4, result);
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5);
        assert_eq!(30, result);
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6);
        assert_eq!(23, result);
    }
}
