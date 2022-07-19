// 118. Pascal's Triangle
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut result = vec![vec![1]];
        for i in 1..num_rows {
            let mut tmp = vec![1; i + 1];
            for j in 1..i {
                tmp[j] = result[i - 1][j - 1] + result[i - 1][j];
            }
            result.push(tmp);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::generate(5);
        assert_eq!(
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ],
            result
        )
    }

    #[test]
    fn test_case_02() {
        let result = Solution::generate(1);
        assert_eq!(vec![vec![1]], result)
    }
}
