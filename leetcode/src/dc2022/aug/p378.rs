// 378. Kth Smallest Element in a Sorted Matrix

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let mut left = matrix[0][0];
        let mut right = matrix[n - 1][n - 1];
        while left <= right {
            let mid = (left + right) / 2;
            if Self::count_less_than_equal(&matrix, mid) >= k {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    fn count_less_than_equal(matrix: &Vec<Vec<i32>>, v: i32) -> i32 {
        let mut j = matrix.len();
        let mut count = 0;
        for i in 0..matrix.len() {
            while j >= 1 && v < matrix[i][j - 1] {
                j -= 1;
            }
            count += j;
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result =
            Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8);
        assert_eq!(13, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::kth_smallest(vec![vec![-5]], 1);
        assert_eq!(-5, result)
    }
}
