#[cfg(test)]
mod tests {
    use super::solution::candy;

    #[test]
    fn test_case_01() {
        let result = candy(vec![1, 0, 2]);
        assert_eq!(5, result)
    }

    #[test]
    fn test_case_02() {
        let result = candy(vec![1, 2, 2]);
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_03() {
        let result = candy(vec![100]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_04() {
        let result = candy(vec![1, 3, 4, 5, 2]);
        assert_eq!(11, result)
    }
}

// 135. Candy
mod solution {
    use std::cmp::Ordering;
    #[allow(dead_code)]
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut sum = 1;
        let mut up = 0;
        let mut down = 0;
        let mut peak = 0;
        for i in 1..ratings.len() {
            match ratings[i].cmp(&ratings[i - 1]) {
                Ordering::Less => {
                    sum += down + 1;
                    down += 1;
                    if peak < down {
                        sum += 1;
                    }
                    up = 0;
                }
                Ordering::Equal => {
                    sum += 1;
                    up = 0;
                    down = 0;
                    peak = 0;
                }
                Ordering::Greater => {
                    up += 1;
                    sum += up + 1;
                    peak = up;
                    down = 0;
                }
            }
        }
        sum
    }
}
