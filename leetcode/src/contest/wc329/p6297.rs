//  6297. Sort the Students by Their Kth Score

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        score.sort_by(|s1, s2| s2[k].cmp(&s1[k]));
        score
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::sort_the_students(
            vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]],
            2,
        );
        assert_eq!(
            vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]],
            result
        )
    }
}
