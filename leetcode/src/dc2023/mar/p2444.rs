// 2444. Count Subarrays With Fixed Bounds

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut index_of_out_of_range = -1;
        let mut index_of_min_k = -1;
        let mut index_of_max_k = -1;
        let mut result = 0;
        for (i, n) in nums.into_iter().enumerate() {
            if n < min_k || max_k < n {
                index_of_out_of_range = i as i64;
            } else {
                if n == min_k {
                    index_of_min_k = i as i64;
                }
                if n == max_k {
                    index_of_max_k = i as i64;
                }
            }
            result += (index_of_min_k.min(index_of_max_k) - index_of_out_of_range).max(0);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1);
        assert_eq!(10, result)
    }
}
