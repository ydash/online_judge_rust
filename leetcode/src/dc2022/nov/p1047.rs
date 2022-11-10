// 1047. Remove All Adjacent Duplicates In String

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = vec![];
        for b in s.as_bytes() {
            if !stack.is_empty() && stack.last().unwrap() == b {
                stack.pop();
            } else {
                stack.push(*b)
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
        let result = Solution::remove_duplicates("abbaca".to_owned());
        assert_eq!("ca", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::remove_duplicates("azxxzy".to_owned());
        assert_eq!("ay", result)
    }
}
