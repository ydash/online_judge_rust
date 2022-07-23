// 315. Count of Smaller Numbers After Self

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; nums.len()];
        let mut bit = BIT::new(20001);
        for i in (0..nums.len()).rev() {
            let j = nums[i] + 10001;
            bit.add(j, 1);
            count[i] = bit.sum(j - 1);
        }
        count
    }
}

struct BIT {
    array: Vec<i32>,
    n: usize,
}

impl BIT {
    #[inline]
    fn new(n: usize) -> BIT {
        BIT {
            array: vec![0; n + 1],
            n,
        }
    }

    fn sum(&self, mut i: i32) -> i32 {
        let mut acc = 0;
        while i > 0 {
            acc += self.array[i as usize];
            i -= i & -i;
        }
        acc
    }

    fn add(&mut self, mut i: i32, v: i32) {
        let n = self.n as i32;
        while i <= n {
            self.array[i as usize] += v;
            i += i & -i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::count_smaller(vec![5, 2, 6, 1]);
        assert_eq!(vec![2, 1, 1, 0], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::count_smaller(vec![-1]);
        assert_eq!(vec![0], result)
    }

    #[test]
    fn test_case_03() {
        let result = Solution::count_smaller(vec![1000, -1000]);
        assert_eq!(vec![1, 0], result)
    }
}
