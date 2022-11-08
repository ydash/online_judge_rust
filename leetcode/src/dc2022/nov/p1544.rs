// 1544. Make The String Great

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn make_good(s: String) -> String {
        let ng_diff = (b'a' - b'A') as i32;
        let mut stack = vec![];
        for &b in s.as_bytes() {
            if !stack.is_empty() && (stack[stack.len() - 1] as i32 - b as i32).abs() == ng_diff {
                stack.pop();
            } else {
                stack.push(b);
            }
        }
        String::from_utf8(stack).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::make_good("leEeetcode".to_owned());
        assert_eq!("leetcode", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::make_good("abBAcC".to_owned());
        assert_eq!("", result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::make_good("s".to_owned());
        assert_eq!("s", result)
    }
}
