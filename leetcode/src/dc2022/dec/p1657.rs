// 1657. Determine if Two Strings Are Close

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        let mut count1 = vec![0; 26];
        let mut count2 = vec![0; 26];
        for (b1, b2) in word1.bytes().zip(word2.bytes()) {
            count1[usize::from(b1 - b'a')] += 1;
            count2[usize::from(b2 - b'a')] += 1;
        }
        for (&c1, &c2) in count1.iter().zip(count2.iter()) {
            if (c1 == 0) != (c2 == 0) {
                return false;
            }
        }
        count1.sort();
        count2.sort();
        count1 == count2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::close_strings("abc".to_owned(), "bca".to_owned());
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::close_strings("a".to_owned(), "aa".to_owned());
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::close_strings("cabbba".to_owned(), "abbccc".to_owned());
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::close_strings("abbc".to_owned(), "abcd".to_owned());
        assert_eq!(false, result)
    }
}
