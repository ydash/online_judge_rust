#[allow(dead_code)]
#[derive(Debug)]
struct NumArray {
    nums: Vec<i32>,
    sum_array: Vec<i32>,
    size: usize,
}

impl NumArray {
    #[allow(dead_code)]
    fn new(nums: Vec<i32>) -> Self {
        let mut num_array = NumArray {
            nums: vec![0; nums.len()],
            sum_array: vec![0; nums.len() + 1],
            size: nums.len(),
        };
        for i in 0..nums.len() {
            num_array.update(i as i32, nums[i]);
        }

        num_array
    }

    fn update(&mut self, index: i32, val: i32) {
        let diff = val - self.nums[index as usize];
        self.nums[index as usize] = val;
        let mut i = index + 1;
        let lim = self.size as i32;
        while i <= lim {
            self.sum_array[i as usize] += diff;
            i += i & -i;
        }
    }

    #[allow(dead_code)]
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        fn sum(array: &Vec<i32>, mut i: i32) -> i32 {
            let mut acc = 0;
            while i > 0 {
                acc += array[i as usize];
                i -= i & -i
            }
            acc
        }
        sum(&self.sum_array, right + 1) - sum(&self.sum_array, left)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_01() {
        let mut num_array = super::NumArray::new(vec![1, 3, 5]);
        assert_eq!(1, num_array.sum_range(0, 0));
        assert_eq!(4, num_array.sum_range(0, 1));
        assert_eq!(9, num_array.sum_range(0, 2));
        assert_eq!(3, num_array.sum_range(1, 1));
        assert_eq!(8, num_array.sum_range(1, 2));
        assert_eq!(5, num_array.sum_range(2, 2));
        num_array.update(1, 2);
        assert_eq!(1, num_array.sum_range(0, 0));
        assert_eq!(3, num_array.sum_range(0, 1));
        assert_eq!(8, num_array.sum_range(0, 2));
        assert_eq!(2, num_array.sum_range(1, 1));
        assert_eq!(7, num_array.sum_range(1, 2));
        assert_eq!(5, num_array.sum_range(2, 2));
    }
}
