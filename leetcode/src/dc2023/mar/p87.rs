// 87. Scramble String

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_scramble(s1: String, s2: String) -> bool {
        fn aux(s1: &[u8], s2: &[u8], cache: &mut HashMap<String, bool>) -> bool {
            if s1 == s2 {
                return true;
            }
            let key = format!(
                "{} {}",
                std::str::from_utf8(s1).unwrap(),
                std::str::from_utf8(s2).unwrap()
            );
            if let Some(&calculated) = cache.get(&key) {
                return calculated;
            }
            let mut count = [0; 26];
            for (&b1, &b2) in s1.iter().zip(s2) {
                count[(b1 - b'a') as usize] += 1;
                count[(b2 - b'a') as usize] -= 1;
            }
            if count.iter().any(|&c| c != 0) {
                return false;
            }
            for i in 1..s1.len() {
                if aux(&s1[..i], &s2[..i], cache) && aux(&s1[i..], &s2[i..], cache)
                    || aux(&s1[..i], &s2[s2.len() - i..], cache)
                        && aux(&s1[i..], &s2[..s2.len() - i], cache)
                {
                    cache.insert(key.to_string(), true);
                    return true;
                }
            }
            cache.insert(key.to_string(), false);
            false
        }
        aux(s1.as_bytes(), s2.as_bytes(), &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::is_scramble("great".to_owned(), "rgeat".to_owned());
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::is_scramble("abcde".to_owned(), "caebd".to_owned());
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::is_scramble("a".to_owned(), "a".to_owned());
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::is_scramble("abc".to_owned(), "bca".to_owned());
        assert_eq!(true, result)
    }
}
