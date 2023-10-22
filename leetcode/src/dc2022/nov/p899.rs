// 899. Orderly Queue

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn orderly_queue(mut s: String, k: i32) -> String {
        if k < 2 {
            let bytes = s
                .as_bytes()
                .to_owned()
                .into_iter()
                .collect::<Vec<_>>();
            let min = *bytes.iter().min().unwrap();
            (0..s.len())
                .filter(|&i| bytes[i] == min)
                .map(|i| {
                    let mut rotated = bytes[i..].to_vec();
                    rotated.extend_from_slice(&bytes[..i]);
                    let s = String::from_utf8(rotated).unwrap();
                    s
                })
                .min()
                .unwrap()
        } else {
            unsafe {
                s.as_bytes_mut().sort();
            }
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::orderly_queue("cba".to_owned(), 1);
        assert_eq!("acb", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::orderly_queue("baaca".to_owned(), 3);
        assert_eq!("aaabc", result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::orderly_queue("dacaba".to_owned(), 1);
        assert_eq!("abadac", result)
    }
}
