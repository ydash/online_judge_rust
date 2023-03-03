// 28. Find the Index of the First Occurrence in a String

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        fn gen_lps(pattern: &[u8]) -> Vec<usize> {
            let mut lps = vec![0; pattern.len()];
            let mut i = 1;
            let mut suffix_i = 0;
            while i < pattern.len() {
                if pattern[i] == pattern[suffix_i] {
                    lps[i] = suffix_i + 1;
                    i += 1;
                    suffix_i += 1;
                } else {
                    if suffix_i != 0 {
                        suffix_i = lps[suffix_i - 1];
                    } else {
                        i += 1;
                    }
                }
            }
            lps
        }
        let lps = gen_lps(needle.as_bytes());
        let mut i = 0;
        let mut j = 0;
        while i < haystack.len() {
            if haystack.as_bytes()[i] == needle.as_bytes()[j] {
                i += 1;
                j += 1;
                if j == needle.len() {
                    return (i - needle.len()) as i32;
                }
            } else if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
        -1
    }
}
