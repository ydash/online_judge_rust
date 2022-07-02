#[cfg(test)]
mod tests {
    use super::solution::max_area;

    #[test]
    fn test_case_0() {
        let result = max_area(5, 4, vec![1, 2, 4], vec![1, 3]);
        assert_eq!(4, result)
    }
    #[test]
    fn test_case_1() {
        let result = max_area(5, 4, vec![3, 1], vec![1]);
        assert_eq!(6, result)
    }

    #[test]
    fn test_case_2() {
        let result = max_area(5, 4, vec![3], vec![3]);
        assert_eq!(9, result)
    }

    #[test]
    fn test_case3() {
        let result = max_area(1000000000, 1000000000, vec![2], vec![2]);
        assert_eq!(81, result);
    }
}

mod solution {
    #[allow(dead_code)]
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        horizontal_cuts.sort();
        horizontal_cuts.push(h);
        let max_h_width = max_width(&horizontal_cuts);
        vertical_cuts.sort();
        vertical_cuts.push(w);
        let max_v_width = max_width(&vertical_cuts);

        ((i64::from(max_h_width) * i64::from(max_v_width)) % 1000000007) as i32
    }

    fn max_width(v: &Vec<i32>) -> i32 {
        v.iter().fold((0, 0), |acc, &i| (acc.0.max(i - acc.1), i)).0
    }
}
