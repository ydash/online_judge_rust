// 345. Reverse Vowels of a String

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_vowels(mut s: String) -> String {
        let vowels = HashSet::from([b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U']);
        let indices = s
            .as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(i, b)| if vowels.contains(b) { Some(i) } else { None })
            .collect::<Vec<_>>();
        unsafe {
            let bytes = s.as_bytes_mut();
            let lim = indices.len() / 2;
            for i in 0..lim {
                let left = indices[i];
                let right = indices[indices.len() - i - 1];
                let tmp = bytes[left];
                bytes[left] = bytes[right];
                bytes[right] = tmp;
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::reverse_vowels("hello".to_owned());
        assert_eq!("holle", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::reverse_vowels("leetcode".to_owned());
        assert_eq!("leotcede", result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::reverse_vowels("aA".to_owned());
        assert_eq!("Aa", result)
    }
}
