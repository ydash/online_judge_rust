// 491. Non-decreasing Subsequences

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = HashSet::new();
        'outer: for b in 3..(1 << nums.len()) {
            let mut tmp = vec![];
            for i in 0..nums.len() {
                if (1 << i) & b != 0 {
                    if tmp.last().map_or(false, |&n| n > nums[i]) {
                        continue 'outer;
                    }
                    tmp.push(nums[i]);
                }
            }
            if tmp.len() > 1 {
                result.insert(tmp);
            }
        }
        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let mut result = Solution::find_subsequences(vec![4, 6, 7, 7]);
        result.sort();
        assert_eq!(
            vec![
                vec![4, 6],
                vec![4, 6, 7],
                vec![4, 6, 7, 7],
                vec![4, 7],
                vec![4, 7, 7],
                vec![6, 7],
                vec![6, 7, 7],
                vec![7, 7]
            ],
            result
        )
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_subsequences(vec![4, 4, 3, 2, 1]);
        assert_eq!(vec![vec![4, 4]], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::find_subsequences(vec![1]);
        assert_eq!(Vec::<Vec<i32>>::new(), result)
    }
}
