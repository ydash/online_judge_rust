// 869. Reordered Power of 2

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut n_chars: Vec<_> = n.to_string().chars().collect();
        n_chars.sort();
        let n_str = String::from_iter(n_chars);
        let mut current = 1;
        for _ in 0..32 {
            let mut chars: Vec<_> = current.to_string().chars().collect();
            chars.sort();
            let str = String::from_iter(chars);
            if n_str == str {
                return true;
            }
            current <<= 1;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::reordered_power_of2(1);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::reordered_power_of2(10);
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::reordered_power_of2(46);
        assert_eq!(true, result)
    }
}
