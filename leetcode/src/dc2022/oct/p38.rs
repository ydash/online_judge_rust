// 38. Count and Say

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn count_and_say(n: i32) -> String {
        fn aux(str: String) -> String {
            let mut result = String::new();
            let mut chars = str.chars();
            let mut prev = chars.next().unwrap();
            let mut count = 1;
            for c in chars {
                if prev == c {
                    count += 1;
                } else {
                    result.push_str(&count.to_string());
                    result.push(prev);
                    prev = c;
                    count = 1;
                }
            }
            result.push_str(&count.to_string());
            result.push(prev);
            result
        }
        let mut result = "1".to_owned();
        for _ in 1..n {
            result = aux(result);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::count_and_say(1);
        assert_eq!("1", result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::count_and_say(4);
        assert_eq!("1211", result)
    }
}
