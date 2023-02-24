// 1675. Minimize Deviation in Array

use std::collections::BTreeSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut btree_set = BTreeSet::new();
        for n in nums {
            let m = if n % 2 == 0 { n } else { n * 2 };
            btree_set.insert(m);
        }
        let mut result = i32::MAX;
        loop {
            let min = *btree_set.iter().min().unwrap();
            let max = *btree_set.iter().max().unwrap();
            result = result.min(max - min);
            if max % 2 == 0 {
                btree_set.remove(&max);
                btree_set.insert(max / 2);
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
        let result = Solution::minimum_deviation(vec![1, 2, 3, 4]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::minimum_deviation(vec![4, 1, 5, 20, 3]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::minimum_deviation(vec![2, 10, 8]);
        assert_eq!(3, result)
    }
}
