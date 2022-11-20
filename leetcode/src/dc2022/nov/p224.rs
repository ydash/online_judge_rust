// 224. Basic Calculator

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn calculate(s: String) -> i32 {
        let mut result = 0;
        let mut num = 0;
        let mut unary = 1;
        let mut stack = vec![];
        for c in s.chars() {
            dbg!(c, result, num, unary);
            match c {
                d if ('0'..='9').contains(&d) => num = num * 10 + (d.to_digit(10).unwrap() as i32),
                '(' => {
                    stack.push((result, unary));
                    result = 0;
                    unary = 1;
                }
                ')' => {
                    let (n, u) = stack.pop().unwrap();
                    result = n + u * (result + unary * num);
                    unary = 1;
                    num = 0;
                }
                '+' => {
                    result += unary * num;
                    num = 0;
                    unary = 1
                }
                '-' => {
                    result += unary * num;
                    num = 0;
                    unary = -1
                }
                _ => (),
            }
        }
        result + unary * num
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::calculate("1 + 1".to_owned());
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::calculate("2-10 + 2 ".to_owned());
        assert_eq!(-6, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::calculate("(1-(4+5-2)+3)-(6+8)".to_owned());
        assert_eq!(-17, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::calculate("10 + 14 - 18".to_owned());
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::calculate("     99     ".to_owned());
        assert_eq!(99, result)
    }
}
