// 134. Gas Station

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (rest, start, suffix) = gas.into_iter().zip(cost.into_iter()).enumerate().fold(
            (0, 0, 0),
            |(rest, start, suffix), (i, (gas, cost))| {
                let rest = rest + gas - cost;
                if rest < 0 {
                    (0, i + 1, rest + suffix)
                } else {
                    (rest, start, suffix)
                }
            },
        );
        if rest + suffix < 0 {
            -1
        } else {
            start as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]);
        assert_eq!(-1, result)
    }
}
