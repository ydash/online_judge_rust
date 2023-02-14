// 67. Add Binary

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();
        let mut carry_up = 0;
        let a = a.as_bytes().iter().rev().collect::<Vec<_>>();
        let b = b.as_bytes().iter().rev().collect::<Vec<_>>();
        for i in 0..a.len().max(b.len()) {
            let x = a.get(i).map_or(0, |&b| b - b'0');
            let y = b.get(i).map_or(0, |&b| b - b'0');
            let current = (x + y + carry_up) % 2;
            result.push(if current == 0 { '0' } else { '1' });
            carry_up = (x + y + carry_up) / 2;
        }
        if carry_up == 1 {
            result.push('1');
        }
        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::add_binary("11".to_owned(), "1".to_owned());
        assert_eq!("100", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::add_binary("1010".to_owned(), "1011".to_owned());
        assert_eq!("10101", result)
    }
}
