// 383. Ransom Note

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count = vec![0; (b'a'..=b'z').len()];
        for i in magazine.as_bytes().iter().map(|b| usize::from(b - b'a')) {
            count[i] += 1;
        }
        for i in ransom_note.as_bytes().iter().map(|b| usize::from(b - b'a')) {
            if count[i] < 1 {
                return false;
            }
            count[i] -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::can_construct("a".to_owned(), "b".to_owned());
        assert_eq!(result, false)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::can_construct("aa".to_owned(), "ab".to_owned());
        assert_eq!(result, false)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::can_construct("aa".to_owned(), "aab".to_owned());
        assert_eq!(result, true)
    }
}
