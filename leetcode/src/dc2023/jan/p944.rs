// 944. Delete Columns to Make Sorted

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let m = strs[0].len();
        let mut result = 0;
        for i in 0..m {
            let mut prev = b'a';
            for j in 0..n {
                let current = strs[j].as_bytes()[i];
                if prev <= current {
                    prev = current;
                } else {
                    result += 1;
                    break;
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
        let result =
            Solution::min_deletion_size(vec!["cba".to_owned(), "daf".to_owned(), "ghi".to_owned()]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::min_deletion_size(vec!["a".to_owned(), "b".to_owned()]);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_03() {
        let result =
            Solution::min_deletion_size(vec!["zyx".to_owned(), "wvu".to_owned(), "tsr".to_owned()]);
        assert_eq!(3, result)
    }
}
