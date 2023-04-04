// 2405. Optimal Partition of String

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn partition_string(s: String) -> i32 {
        let mut seen = [false; 26];
        let mut result = 1;
        for &b in s.as_bytes() {
            let i = usize::from(b - b'a');
            if seen[i] {
                for j in 0..26 {
                    seen[j] = false;
                }
                result += 1;
            }
            seen[i] = true;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::partition_string("abacaba".to_owned());
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::partition_string("ssssss".to_owned());
        assert_eq!(6, result)
    }
}
