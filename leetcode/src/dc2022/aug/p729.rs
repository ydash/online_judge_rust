// 729. My Calendar I

use std::collections::BTreeMap;

struct MyCalendar {
    events: BTreeMap<i32, i32>,
}
#[allow(dead_code)]
impl MyCalendar {
    fn new() -> Self {
        MyCalendar {
            events: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let prev = self.events.range(0..start).next_back();
        let next = self.events.range(start..).next();
        let can_book = match (prev, next) {
            (None, None) => true,
            (None, Some((&n_start, _))) => end <= n_start,
            (Some((_, &p_end)), None) => p_end <= start,
            (Some((_, &p_end)), Some((&n_start, _))) => p_end <= start && end <= n_start,
        };
        if can_book {
            self.events.insert(start, end);
        }
        can_book
    }
}

#[cfg(test)]
mod tests {
    use super::MyCalendar;

    #[test]
    fn test_case_01() {
        let mut calender = MyCalendar::new();
        assert!(calender.book(10, 20));
        assert!(!calender.book(10, 15));
        assert!(calender.book(5, 10));
        assert!(!calender.book(15, 25));
        assert!(calender.book(30, 40));
        assert!(!calender.book(25, 35));
        assert!(calender.book(20, 30));
    }
}
