// 1143. Longest Common Subsequence

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let n = text1.len();
        let m = text2.len();
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        let mut prev = vec![0; m + 1];
        let mut current = vec![0; m + 1];
        for i in 0..n {
            for j in 0..m {
                current[j + 1] = prev[j];
                if text1[i] == text2[j] {
                    current[j + 1] += 1;
                }
                current[j + 1] = current[j + 1].max(current[j]).max(prev[j + 1]);
            }
            std::mem::swap(&mut prev, &mut current);
        }
        prev[m]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::longest_common_subsequence("abcde".to_owned(), "ace".to_owned());
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::longest_common_subsequence("abc".to_owned(), "abc".to_owned());
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::longest_common_subsequence("abc".to_owned(), "def".to_owned());
        assert_eq!(0, result)
    }
}
