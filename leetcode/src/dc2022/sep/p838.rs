// 838. Push Dominoes

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn push_dominoes(dominoes: String) -> String {
        let bytes = dominoes.as_bytes();
        let mut left = vec![-1; dominoes.len()];
        let mut right = vec![-1; dominoes.len()];
        let mut p = -1;
        for i in 0..dominoes.len() {
            if bytes[i] == b'R' {
                p = i as i32;
            } else if bytes[i] == b'L' {
                p = -1;
            }
            right[i] = p;
        }
        p = -1;
        for i in (0..bytes.len()).rev() {
            if bytes[i] == b'L' {
                p = i as i32;
            } else if bytes[i] == b'R' {
                p = -1;
            }
            left[i] = p;
        }
        let mut result = String::new();
        for i in 0..bytes.len() {
            if left[i] == -1 && right[i] == -1 {
                result.push('.');
            } else if left[i] == -1 {
                result.push('R');
            } else if right[i] == -1 {
                result.push('L');
            } else {
                let ld = left[i] - i as i32;
                let rd = i as i32 - right[i];
                if ld == rd {
                    result.push('.');
                } else if ld < rd {
                    result.push('L');
                } else {
                    result.push('R');
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::push_dominoes("RR.L".to_owned());
        assert_eq!("RR.L".to_owned(), result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::push_dominoes(".L.R...LR..L..".to_owned());
        assert_eq!("LL.RR.LLRRLL..".to_owned(), result)
    }
}
