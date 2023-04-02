// 2300. Successful Pairs of Spells and Potions

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        fn bin_search(v: &[i32], target: i64, mul: i64) -> usize {
            let mut left = 0;
            let mut right = v.len();
            while left < right {
                let mid = left + (right - left) / 2;
                if v[mid] as i64 * mul < target {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        }
        potions.sort();
        let mut result = vec![];
        for spell in spells {
            result.push((potions.len() - bin_search(&potions, success, spell.into())) as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7);
        assert_eq!(vec![4, 0, 3], result);
    }

    #[test]
    fn test_case_02() {
        let result = Solution::successful_pairs(vec![1], vec![4], 4);
        assert_eq!(vec![1], result);
    }

    #[test]
    fn test_case_03() {
        let result = Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16);
        assert_eq!(vec![2, 0, 2], result);
    }
}
