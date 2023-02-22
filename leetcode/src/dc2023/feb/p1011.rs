// 1011. Capacity To Ship Packages Within D Days

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut left = *weights.iter().max().unwrap();
        let mut right = weights.iter().sum::<i32>();
        while left < right {
            let mid = left + (right - left) / 2;
            let count = weights
                .iter()
                .fold((0, 1), |(acc, c), &x| {
                    if acc + x > mid {
                        (x, c + 1)
                    } else {
                        (acc + x, c)
                    }
                })
                .1;
            if count > days {
                left = mid + 1;
            } else {
                right = mid;
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
        let result = Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5);
        assert_eq!(15, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::ship_within_days(vec![42], 1);
        assert_eq!(42, result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1);
        assert_eq!(55, result)
    }
}
