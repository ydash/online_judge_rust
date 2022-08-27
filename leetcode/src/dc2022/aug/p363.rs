// 363. Max Sum of Rectangle No Larger Than K

use std::collections::BTreeSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut result = i32::MIN;
        for l in 0..n {
            let mut sum_of_row = vec![0; m];
            for r in l..n {
                for i in 0..m {
                    sum_of_row[i] += matrix[i][r];
                }
                let mut cache = BTreeSet::from([0]);
                let mut sum1 = 0;
                for s in sum_of_row.iter() {
                    sum1 += s;
                    let mut x = cache.range(sum1 - k..);
                    x.next().map(|sum2| result = result.max(sum1 - sum2));
                    cache.insert(sum1);
                }
            }
            if result == k {
                return k;
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
        let result = Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 3);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_03() {
        let result =
            Solution::max_sum_submatrix(vec![vec![1, 0, -1], vec![2, 5, 8], vec![-5, 3, -9]], 7);
        assert_eq!(7, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 0);
        assert_eq!(-1, result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::max_sum_submatrix(
            vec![vec![5, -4, -3, 4], vec![-3, -4, 4, 5], vec![5, 1, 5, -4]],
            10,
        );
        assert_eq!(10, result)
    }
}
