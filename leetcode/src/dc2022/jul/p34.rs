// 34. Find First and Last Position of Element in Sorted Array

use std::cmp::Ordering;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![-1, -1];
        if nums.is_empty() {
            return result;
        }
        let start = Self::binary_search(&nums, target, |v, mid, _left, right| {
            if mid == 0 || v[mid - 1] != target {
                Some(mid as i32)
            } else {
                *right = mid;
                None
            }
        });
        if start != -1 {
            let end = Self::binary_search(&nums, target, |v, mid, left, _right| {
                if mid == v.len() - 1 || v[mid + 1] != target {
                    Some(mid as i32)
                } else {
                    *left = mid + 1;
                    None
                }
            });
            result[0] = start;
            result[1] = end;
        }

        result
    }

    fn binary_search<F: Fn(&Vec<i32>, usize, &mut usize, &mut usize) -> Option<i32>>(
        nums: &Vec<i32>,
        target: i32,
        f: F,
    ) -> i32 {
        let mut size = nums.len();
        let mut left = 0;
        let mut right = size;
        while left < right {
            let mid = left + size / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => match f(&nums, mid, &mut left, &mut right) {
                    Some(result) => return result,
                    None => (),
                },
                Ordering::Greater => right = mid,
            }
            size = right - left;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8);
        assert_eq!(vec![3, 4], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6);
        assert_eq!(vec![-1, -1], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::search_range(vec![], 0);
        assert_eq!(vec![-1, -1], result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::search_range(vec![-3, -1, 3, 4], -3);
        assert_eq!(vec![0, 0], result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::search_range(vec![1; 10], 1);
        assert_eq!(vec![0, 9], result)
    }

    #[test]
    fn test_case_06() {
        let result = Solution::search_range(vec![1], 0);
        assert_eq!(vec![-1, -1], result)
    }
}
