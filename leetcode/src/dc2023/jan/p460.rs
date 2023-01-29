// 460. LFU Cache

use std::collections::{HashMap, VecDeque};

struct LFUCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    count: HashMap<i32, i32>,
    count_to_keys: HashMap<i32, VecDeque<i32>>,
    min_count: i32,
}

impl LFUCache {
    #[allow(dead_code)]
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            cache: HashMap::new(),
            count: HashMap::new(),
            count_to_keys: HashMap::new(),
            min_count: 0,
        }
    }

    #[allow(dead_code)]
    fn get(&mut self, key: i32) -> i32 {
        match self.cache.get(&key) {
            None => -1,
            Some(&v) => {
                let count = self.count.get_mut(&key).unwrap();
                let keys = self.count_to_keys.get_mut(count).unwrap();
                keys.iter().position(|&x| x == key).map(|i| keys.remove(i));
                if *count == self.min_count && keys.is_empty() {
                    self.min_count += 1;
                }
                *count += 1;
                self.count_to_keys
                    .entry(*count)
                    .or_insert(VecDeque::new())
                    .push_back(key);
                v
            }
        }
    }

    #[allow(dead_code)]
    fn put(&mut self, key: i32, value: i32) {
        if self.capacity <= 0 {
            return;
        }
        match self.cache.get(&key) {
            Some(_) => {
                self.cache.insert(key, value);
                self.get(key);
            }
            None => {
                if self.cache.len() >= self.capacity {
                    self.count_to_keys
                        .get_mut(&self.min_count)
                        .and_then(|keys| keys.pop_front())
                        .map(|v| self.cache.remove(&v));
                }
                self.cache.insert(key, value);
                self.count.insert(key, 1);
                self.min_count = 1;
                self.count_to_keys
                    .entry(1)
                    .or_insert(VecDeque::new())
                    .push_back(key);
            }
        }
    }
}
