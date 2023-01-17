// 926. Flip String to Monotone Increasing

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_flips_mono_incr(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .fold((0, 0), |(count_one, count_flip), &b| {
                if b == b'1' {
                    (count_one + 1, count_flip)
                } else {
                    (count_one, count_one.min(count_flip + 1))
                }
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::min_flips_mono_incr("00110".to_owned());
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_flips_mono_incr("010110".to_owned());
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::min_flips_mono_incr("1111001110".to_owned());
        assert_eq!(3, result)
    }
}
