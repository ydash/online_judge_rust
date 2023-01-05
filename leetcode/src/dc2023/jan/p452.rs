// 452. Minimum Number of Arrows to Burst Balloons

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut x = points[0][1];
        let mut result = 1;
        for p in points.iter().skip(1) {
            let s = p[0];
            let e = p[1];
            if x < s {
                result += 1;
                x = e;
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
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]);
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_02() {
        let result =
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_03() {
        let result =
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]);
        assert_eq!(2, result)
    }
}
