// 387. First Unique Character in a String

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn first_uniq_char(s: String) -> i32 {
        let mut freq = vec![0; 26];
        for b in s.bytes() {
            freq[usize::from(b - b'a')] += 1;
        }
        for (i, b) in s.bytes().enumerate() {
            if freq[usize::from(b - b'a')] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::first_uniq_char("leetcode".to_string());
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::first_uniq_char("loveleetcode".to_string());
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::first_uniq_char("aabb".to_string());
        assert_eq!(-1, result)
    }
}
