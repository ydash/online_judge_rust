// 380. Insert Delete GetRandom O(1)

use std::collections::HashMap;

struct RandomizedSet {
    nums: Vec<i32>,
    indices: HashMap<i32, usize>,
}

impl RandomizedSet {
    #[allow(dead_code)]
    fn new() -> Self {
        RandomizedSet {
            nums: vec![],
            indices: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn insert(&mut self, val: i32) -> bool {
        if self.indices.contains_key(&val) {
            false
        } else {
            self.nums.push(val);
            self.indices.insert(val, self.nums.len() - 1);
            true
        }
    }

    #[allow(dead_code)]
    fn remove(&mut self, val: i32) -> bool {
        if let Some(i) = self.indices.remove(&val) {
            self.nums[i] = self.nums[self.nums.len() - 1];
            self.indices.get_mut(&self.nums[i]).map(|v| *v = i);
            self.nums.pop();
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn get_random(&self) -> i32 {
        self.nums[rand::random::<usize>() % self.nums.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::RandomizedSet;

    #[test]
    fn test_case_01() {
        let mut randomized_set = RandomizedSet::new();
        assert_eq!(true, randomized_set.insert(0));
        assert_eq!(true, randomized_set.insert(1));
        assert_eq!(false, randomized_set.remove(2));
        assert_eq!(true, randomized_set.remove(0));
        assert_eq!(true, randomized_set.insert(2));
        assert!((1..=2).contains(&randomized_set.get_random()));
        assert_eq!(true, randomized_set.remove(1));
        assert_eq!(2, randomized_set.get_random());
    }
}
