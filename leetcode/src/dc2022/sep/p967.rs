// 967. Numbers With Same Consecutive Differences

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        fn aux(current: usize, last: usize, k: usize, rem: i32, acc: &mut Vec<i32>) {
            if rem == 0 {
                acc.push(current as i32);
                return;
            }
            if last >= k {
                let a = last - k;
                aux(current * 10 + a, a, k, rem - 1, acc)
            }
            let b = last + k;
            if k > 0 && b < 10 {
                aux(current * 10 + b, b, k, rem - 1, acc)
            }
        }
        let mut result = vec![];
        for i in 1..=9 {
            aux(i, i, k as usize, n - 1, &mut result)
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::nums_same_consec_diff(3, 7);
        assert_eq!(vec![181, 292, 707, 818, 929], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::nums_same_consec_diff(2, 1);
        assert_eq!(
            vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98],
            result
        )
    }

    #[test]
    fn test_case_03() {
        let result = Solution::nums_same_consec_diff(2, 0);
        assert_eq!(vec![11, 22, 33, 44, 55, 66, 77, 88, 99], result)
    }
}
