// 912. Sort an Array

struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        fn heapify(arr: &mut [i32]) {
            for i in 1..arr.len() {
                let mut j = i;
                while j > 0 && arr[j] > arr[(j - 1) / 2] {
                    arr.swap(j, (j - 1) / 2);
                    j = (j - 1) / 2;
                }
            }
        }
        fn maintain_heap(arr: &mut [i32]) {
            let mut current = 0;
            while current * 2 + 1 < arr.len() {
                let left = 2 * current + 1;
                let right = left + 1;
                let largest_child = if right < arr.len() && arr[left] < arr[right] {
                    right
                } else {
                    left
                };
                if arr[current] < arr[largest_child] {
                    arr.swap(current, largest_child);
                }
                current = largest_child;
            }
        }
        heapify(&mut nums);
        for i in (1..nums.len()).rev() {
            nums.swap(0, i);
            maintain_heap(&mut nums[0..i])
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_01() {
        let result = Solution::sort_array(vec![5, 2, 3, 1]);
        assert_eq!(vec![1, 2, 3, 5], result)
    }

    #[test]
    fn test_case_02() {
        let result = Solution::sort_array(vec![1, 2, 3, 4, 5]);
        assert_eq!(vec![1, 2, 3, 4, 5], result)
    }
}
