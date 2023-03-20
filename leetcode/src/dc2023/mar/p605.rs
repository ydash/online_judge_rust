// 605. Can Place Flowers

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;
        for i in 0..flowerbed.len() {
            if flowerbed[i] == 0
                && (i == 0 || flowerbed[i - 1] == 0)
                && (i + 1 >= flowerbed.len() || flowerbed[i + 1] == 0)
            {
                count += 1;
                flowerbed[i] = 1;
            }
        }
        count >= n
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1);
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2);
        assert_eq!(false, result)
    }
}
