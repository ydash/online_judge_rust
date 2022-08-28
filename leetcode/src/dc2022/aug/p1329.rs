// 1329. Sort the Matrix Diagonally

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut diag = HashMap::new();
        for i in 0..m {
            for j in 0..n {
                diag.entry(i as i32 - j as i32)
                    .or_insert(BinaryHeap::new())
                    .push(Reverse(mat[i][j]));
            }
        }
        for i in 0..m {
            for j in 0..n {
                mat[i][j] = diag
                    .get_mut(&(i as i32 - j as i32))
                    .and_then(|pq| pq.pop())
                    .map(|elm| elm.0)
                    .unwrap();
            }
        }

        mat
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::diagonal_sort(vec![
            vec![11, 25, 66, 1, 69, 7],
            vec![23, 55, 17, 45, 15, 52],
            vec![75, 31, 36, 44, 58, 8],
            vec![22, 27, 33, 25, 68, 4],
            vec![84, 28, 14, 11, 5, 50],
        ]);
        assert_eq!(
            vec![
                vec![5, 17, 4, 1, 52, 7],
                vec![11, 11, 25, 45, 8, 69],
                vec![14, 23, 25, 44, 58, 15],
                vec![22, 27, 31, 36, 50, 66],
                vec![84, 28, 75, 33, 55, 68]
            ],
            result
        )
    }
}
