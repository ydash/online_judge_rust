// 150. Evaluate Reverse Polish Notation

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        fn arithmetic_op(operator: &str, a: i32, b: i32) -> i32 {
            match operator {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => panic!("Illegal operator: {}", operator),
            }
        }
        let mut stack = vec![];
        for token in tokens.iter() {
            match token.parse::<i32>() {
                Ok(n) => stack.push(n),
                Err(_) => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(arithmetic_op(token.as_str(), a, b));
                }
            }
        }
        stack[0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::eval_rpn(vec!["1".to_owned()]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::eval_rpn(vec![
            "10".to_owned(),
            "6".to_owned(),
            "9".to_owned(),
            "3".to_owned(),
            "+".to_owned(),
            "-11".to_owned(),
            "*".to_owned(),
            "/".to_owned(),
            "*".to_owned(),
            "17".to_owned(),
            "+".to_owned(),
            "5".to_owned(),
            "+".to_owned(),
        ]);
        assert_eq!(22, result)
    }
}
