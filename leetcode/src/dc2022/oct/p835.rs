// 835. Image Overlap

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let mut result = 0;
        let mut arr = vec![vec![0; 3 * n - 2]; 3 * n - 2];
        let mut max = 0;
        for i in 0..n {
            for j in 0..n {
                arr[i + n - 1][j + n - 1] = img1[i][j];
                if img1[i][j] == 1 {
                    max += 1;
                }
            }
        }
        for x in 0..2 * n - 1 {
            for y in 0..2 * n - 1 {
                let mut count = 0;
                for i in 0..n {
                    for j in 0..n {
                        if arr[i + x][j + y] == 1 && img2[i][j] == 1 {
                            count += 1;
                        }
                    }
                }
                result = result.max(count);
                if result == max {
                    return result;
                }
            }
        }
        println!("{:?}", arr);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::largest_overlap(
            vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
            vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]],
        );
        assert_eq!(3, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::largest_overlap(vec![vec![1]], vec![vec![1]]);
        assert_eq!(1, result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::largest_overlap(vec![vec![0]], vec![vec![0]]);
        assert_eq!(0, result)
    }
}
