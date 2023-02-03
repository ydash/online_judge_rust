// 6. Zigzag Conversion

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let mut row = vec![String::new(); num_rows];
        let mut iter = s.chars().peekable();
        while iter.peek().is_some() {
            for i in 0..num_rows {
                iter.next().map(|c| row[i].push(c));
            }
            for i in (1..num_rows - 1).rev() {
                iter.next().map(|c| row[i].push(c));
            }
        }
        row.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::convert("PAYPALISHIRING".to_owned(), 3);
        assert_eq!("PAHNAPLSIIGYIR", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::convert("PAYPALISHIRING".to_owned(), 4);
        assert_eq!("PINALSIGYAHRPI", result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::convert("ABCD".to_owned(), 1);
        assert_eq!("ABCD", result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::convert("ABCD".to_owned(), 2);
        assert_eq!("ACBD", result)
    }
}
