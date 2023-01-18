// 918. Maximum Sum Circular Subarray

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut acc_max = 0;
        let mut acc_min = 0;
        let mut max_array = i32::MIN;
        let mut min_array = i32::MAX;
        for n in nums {
            acc_max = n.max(n + acc_max);
            acc_min = n.min(n + acc_min);
            max_array = max_array.max(acc_max);
            min_array = min_array.min(acc_min);
            sum += n;
        }
        if sum != min_array {
            max_array.max(sum - min_array)
        } else {
            max_array
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::max_subarray_sum_circular(vec![1, 2, 3, 4, 5]);
        assert_eq!(15, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::max_subarray_sum_circular(vec![-3, -2, -1, -4]);
        assert_eq!(-1, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::max_subarray_sum_circular(vec![1, 3, -5, 3, 6]);
        assert_eq!(13, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::max_subarray_sum_circular(vec![5, -3, 5]);
        assert_eq!(10, result)
    }
}
