#[cfg(test)]
mod tests {
    use super::solution::is_interleave;

    #[test]
    fn test_case_01() {
        let result = is_interleave(
            String::from("aabcc"),
            String::from("dbbca"),
            String::from("aadbbcbcac"),
        );
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = is_interleave(
            String::from("aabcc"),
            String::from("dbbca"),
            String::from("aadbbbaccc"),
        );
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = is_interleave(String::from(""), String::from(""), String::from(""));
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_04() {
        let result = is_interleave(String::from(""), String::from(""), String::from("a"));
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_05() {
        let result = is_interleave(String::from("abc"), String::from(""), String::from("abc"));
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_06() {
        let result = is_interleave(String::from(""), String::from("abc"), String::from("abc"));
        assert_eq!(true, result)
    }
}

// 97. Interleaving String
mod solution {
    #[allow(dead_code)]
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let m = s1.len();
        let n = s2.len();
        if m + n != s3.len() {
            return false;
        }
        let c1 = s1.as_bytes();
        let c2 = s2.as_bytes();
        let c3 = s3.as_bytes();
        let mut prev = vec![false; n + 1];
        let mut current = vec![false; n + 1];
        for i in 0..=m {
            for j in 0..=n {
                current[j] = (i == 0 && j == 0)
                    || (j > 0 && current[j - 1] && c2[j - 1] == c3[i + j - 1])
                    || (prev[j] && c1[i - 1] == c3[i + j - 1]);
            }
            std::mem::swap(&mut current, &mut prev)
        }
        *prev.last().unwrap()
    }
}
