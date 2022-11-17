// 223. Rectangle Area

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let mut result = (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1);
        let x2 = ax2.min(bx2);
        let x1 = ax1.max(bx1);
        let y2 = ay2.min(by2);
        let y1 = ay1.max(by1);
        if x2 > x1 && y2 > y1 {
            result -= (x2 - x1) * (y2 - y1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2);
        assert_eq!(45, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2);
        assert_eq!(16, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::compute_area(-2, -2, 2, 2, -3, 0, 0, 3);
        assert_eq!(21, result)
    }

    #[test]
    fn test_case_04() {
        let result = Solution::compute_area(-2, -2, 2, 2, -3, -3, 0, 0);
        assert_eq!(21, result)
    }

    #[test]
    fn test_case_05() {
        let result = Solution::compute_area(-2, -2, 2, 2, 0, 0, 3, 3);
        assert_eq!(21, result)
    }

    #[test]
    fn test_case_06() {
        let result = Solution::compute_area(-2, -2, -1, -1, 1, 1, 2, 2);
        assert_eq!(2, result)
    }
}
