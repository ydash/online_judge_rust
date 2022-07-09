#[cfg(test)]
mod tests {
    use super::solution::max_result;

    #[test]
    fn test_case_01() {
        let result = max_result(vec![1, -1, -2, 4, -7, 3], 2);
        assert_eq!(7, result)
    }

    #[test]
    fn test_case_02() {
        let result = max_result(vec![10, -5, -2, 4, 0, 3], 3);
        assert_eq!(17, result)
    }

    #[test]
    fn test_case_03() {
        let result = max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2);
        assert_eq!(0, result)
    }

    #[test]
    fn test_case_04() {
        let result = max_result(vec![8, 34], 1);
        assert_eq!(42, result)
    }
}

// 1696. Jump Game VI
mod solution {
    use std::collections::BinaryHeap;

    #[derive(Debug, PartialEq, Eq, PartialOrd)]
    struct State {
        cost: i32,
        pos: usize,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.cost.cmp(&other.cost)
        }
    }

    #[allow(dead_code)]
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = BinaryHeap::new();
        dp.push(State {
            cost: nums[0],
            pos: 0,
        });
        for current in 1..nums.len() {
            if current > k {
                while dp.peek().map(|s| s.pos < current - k).unwrap_or(false) {
                    dp.pop();
                }
            }
            dp.peek()
                .map(|s| State {
                    cost: s.cost + nums[current],
                    pos: current,
                })
                .map(|s| dp.push(s));
        }

        while dp.peek().map(|s| s.pos != nums.len() - 1).unwrap_or(false) {
            dp.pop();
        }
        dp.pop().unwrap().cost
    }
}
