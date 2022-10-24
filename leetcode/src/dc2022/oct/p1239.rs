// 1239. Maximum Length of a Concatenated String with Unique Characters

use std::collections::HashSet;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_length(arr: Vec<String>) -> i32 {
        fn aux(
            i: usize,
            acc_bit: i32,
            acc_len: usize,
            strs: &[&String],
            bits: &[i32],
            result: &mut usize,
        ) {
            if i >= strs.len() {
                *result = (*result).max(acc_len);
            } else {
                let new_acc_bit = acc_bit | bits[i];
                if new_acc_bit != acc_bit ^ bits[i] {
                    *result = (*result).max(acc_len);
                } else {
                    let new_acc_len = acc_len + strs[i].len();
                    for j in i + 1..=strs.len() {
                        aux(j, new_acc_bit, new_acc_len, strs, bits, result);
                    }
                }
            }
        }
        let arr = arr
            .iter()
            .filter(|s| s.len() == s.as_bytes().iter().collect::<HashSet<_>>().len())
            .collect::<Vec<_>>();
        let bits = arr
            .iter()
            .map(|s| {
                s.as_bytes()
                    .iter()
                    .map(|b| b - b'a')
                    .fold(0, |acc, b| acc | (1 << b))
            })
            .collect::<Vec<_>>();
        let mut result = 0;
        for i in 0..arr.len() {
            aux(i, 0, 0, &arr, &bits, &mut result);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::max_length(vec!["un".to_owned(), "iq".to_owned(), "ue".to_owned()]);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::max_length(vec![
            "cha".to_owned(),
            "r".to_owned(),
            "act".to_owned(),
            "ers".to_owned(),
        ]);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::max_length(vec!["abcdefghijklmnopqrstuvwxyz".to_owned()]);
        assert_eq!(26, result)
    }
}
