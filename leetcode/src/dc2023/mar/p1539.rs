// 1539. Kth Missing Positive Number

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = arr.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] - mid as i32 - 1 >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left as i32 + k
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5);
        assert_eq!(9, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::find_kth_positive(vec![1, 2, 3, 4], 2);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::find_kth_positive(vec![2, 3, 5, 7], 1);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::find_kth_positive(vec![1, 3, 5, 7], 5);
        assert_eq!(9, result)
    }
}
