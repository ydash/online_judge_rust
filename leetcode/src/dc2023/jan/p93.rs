// 93. Restore IP Addresses

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        if s.len() < 4 && 12 < s.len() {
            return vec![];
        }
        fn back_tracking(
            current: usize,
            dot_indices: &mut Vec<usize>,
            s: &str,
            result: &mut Vec<String>,
        ) {
            if dot_indices.len() >= 3 {
                let rest_len = s.len() - current;
                if rest_len > 0
                    && rest_len < 4
                    && (rest_len == 1 || s.as_bytes()[current] != b'0')
                    && s[current..].parse::<i32>().unwrap() <= 255
                {
                    result.push(format!(
                        "{}.{}.{}.{}",
                        String::from_utf8(s.as_bytes()[0..dot_indices[0]].to_vec()).unwrap(),
                        String::from_utf8(s.as_bytes()[dot_indices[0]..dot_indices[1]].to_vec())
                            .unwrap(),
                        String::from_utf8(s.as_bytes()[dot_indices[1]..dot_indices[2]].to_vec())
                            .unwrap(),
                        String::from_utf8(s.as_bytes()[dot_indices[2]..].to_vec()).unwrap(),
                    ));
                }
            } else {
                for i in 1..=3.min(s.len() - current) {
                    if i > 1
                        && (s.as_bytes()[current] == b'0'
                            || s[current..current + i].parse::<i32>().unwrap() > 255)
                    {
                        continue;
                    }
                    dot_indices.push(current + i);
                    back_tracking(current + i, dot_indices, s, result);
                    dot_indices.pop();
                }
            }
        }
        let mut result = vec![];
        back_tracking(0, &mut vec![], &s, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::restore_ip_addresses("25525511135".to_owned());
        assert_eq!(vec!["255.255.11.135", "255.255.111.35"], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::restore_ip_addresses("0000".to_owned());
        assert_eq!(vec!["0.0.0.0"], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::restore_ip_addresses("101023".to_owned());
        assert_eq!(
            vec![
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ],
            result
        );
    }

    #[test]
    fn test_case_04() {
        let result = Solution::restore_ip_addresses("000".to_owned());
        assert_eq!(Vec::<String>::new(), result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::restore_ip_addresses("255255111251".to_owned());
        assert_eq!(vec!["255.255.111.251"], result)
    }

    #[test]
    fn test_case_06() {
        let result = Solution::restore_ip_addresses("2552551112511".to_owned());
        assert_eq!(Vec::<String>::new(), result)
    }
}
