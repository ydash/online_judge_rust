// 2246. Longest Path With Different Adjacent Characters

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let n = parent.len();
        let mut children = vec![vec![]; n];
        for (child, parent) in parent.into_iter().enumerate().skip(1) {
            children[parent as usize].push(child);
        }
        fn dfs(current: usize, children: &[Vec<usize>], label: &[u8], result: &mut i32) -> i32 {
            let mut first_max = 0;
            let mut second_max = 0;
            for &child in children[current].iter() {
                let child_path = dfs(child, children, label, result);
                if label[current] != label[child] && second_max < child_path {
                    second_max = child_path;
                    if second_max > first_max {
                        std::mem::swap(&mut first_max, &mut second_max);
                    }
                }
            }
            *result = (first_max + second_max + 1).max(*result);
            first_max + 1
        }
        let mut result = 0;
        dfs(0, &children, s.as_bytes(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".to_owned());
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::longest_path(vec![-1, 0, 0, 0], "abcd".to_owned());
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::longest_path(vec![-1], "a".to_owned());
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::longest_path(vec![-1, 0, 1, 2, 3, 4], "zzabab".to_owned());
        assert_eq!(5, result)
    }
}
