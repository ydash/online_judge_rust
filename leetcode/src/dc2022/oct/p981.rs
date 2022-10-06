// 981. Time Based Key-Value Store

use std::collections::{BTreeMap, HashMap};

struct TimeMap {
    content: HashMap<String, BTreeMap<i32, String>>,
}

impl TimeMap {
    #[allow(dead_code)]
    fn new() -> Self {
        TimeMap {
            content: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.content
            .entry(key)
            .or_insert(BTreeMap::new())
            .insert(timestamp, value);
    }

    #[allow(dead_code)]
    fn get(&self, key: String, timestamp: i32) -> String {
        self.content
            .get(&key)
            .and_then(|tm| tm.range(..=timestamp).last())
            .map(|e| e.1.to_owned())
            .unwrap_or("".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::TimeMap;

    #[test]
    fn test_case_01() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_owned(), "bar".to_owned(), 1);
        assert_eq!("bar".to_owned(), time_map.get("foo".to_owned(), 1));
        assert_eq!("bar".to_owned(), time_map.get("foo".to_owned(), 3));
        time_map.set("foo".to_owned(), "bar2".to_owned(), 4);
        assert_eq!("bar2".to_owned(), time_map.get("foo".to_owned(), 4));
        assert_eq!("bar2".to_owned(), time_map.get("foo".to_owned(), 5));
        assert_eq!("bar".to_owned(), time_map.get("foo".to_owned(), 3));
    }
}
