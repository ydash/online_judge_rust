// 13. Roman to Integer

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .rfold((0, 0), |(acc, prev), &b| {
                let current = match b {
                    b'I' => 1,
                    b'V' => 5,
                    b'X' => 10,
                    b'L' => 50,
                    b'C' => 100,
                    b'D' => 500,
                    b'M' => 1000,
                    _ => panic!("received unexpected character: {}", b),
                };
                (
                    acc + if current < prev { -current } else { current },
                    current,
                )
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::roman_to_int("III".to_string());
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::roman_to_int("LVIII".to_string());
        assert_eq!(58, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::roman_to_int("MCMXCIV".to_string());
        assert_eq!(1994, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::roman_to_int("IV".to_string());
        assert_eq!(4, result)
    }
}
