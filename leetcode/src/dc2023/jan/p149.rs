// 149. Max Points on a Line

use std::collections::HashMap;

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn max_points(mut points: Vec<Vec<i32>>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        points.sort_by(|a, b| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
            o => o,
        });
        let n = points.len();
        let mut result = 1;
        for i in 0..n {
            let mut line = HashMap::new();
            let x1 = points[i][0];
            let y1 = points[i][1];
            for j in i + 1..n {
                let x2 = points[j][0];
                let y2 = points[j][1];
                let dx = x1 - x2;
                let dy = y1 - y2;
                let d = gcd(dx.abs(), dy.abs());
                let count = line.entry((dx / d, dy / d)).or_insert(1);
                *count += 1;
                result = result.max(*count);
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
        let result = Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]);
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::max_points(vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ]);
        assert_eq!(4, result)
    }
}
