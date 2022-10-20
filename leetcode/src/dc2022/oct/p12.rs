// 12. Integer to Roman

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn int_to_roman(num: i32) -> String {
        let num = num as usize;
        let unit = vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
        let ten = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        let hundred = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        let thousand = vec!["", "M", "MM", "MMM"];
        format!(
            "{}{}{}{}",
            thousand[num / 1000],
            hundred[num % 1000 / 100],
            ten[num % 100 / 10],
            unit[num % 10]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::int_to_roman(3);
        assert_eq!("III", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::int_to_roman(58);
        assert_eq!("LVIII", result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::int_to_roman(1994);
        assert_eq!("MCMXCIV", result)
    }
}
