// 904. Fruit Into Baskets

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut basket = HashMap::new();
        let mut left = 0;
        let mut right = 0;
        while right < fruits.len() {
            *basket.entry(fruits[right]).or_insert(0) += 1;
            if basket.len() > 2 {
                let count = basket.entry(fruits[left]).or_insert(1);
                *count -= 1;
                if *count == 0 {
                    basket.remove(&fruits[left]);
                }
                left += 1;
            }
            right += 1;
        }
        (right - left) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::total_fruit(vec![1, 2, 1]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::total_fruit(vec![0, 1, 1, 2, 3]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::total_fruit(vec![0]);
        assert_eq!(1, result)
    }
}
