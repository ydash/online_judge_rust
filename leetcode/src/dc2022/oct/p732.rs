// 732. My Calendar III

use std::collections::BTreeMap;

struct MyCalendarThree {
    k: i32,
    events: BTreeMap<i32, i32>,
}

impl MyCalendarThree {
    #[allow(dead_code)]
    fn new() -> Self {
        MyCalendarThree {
            k: 0,
            events: BTreeMap::new(),
        }
    }

    #[allow(dead_code)]
    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.events.insert(
            start,
            self.events.range(..=start).last().map_or(0, |e| *e.1),
        );
        self.events
            .insert(end, self.events.range(..=end).last().map_or(0, |e| *e.1));
        for (_, count) in self.events.range_mut(start..end) {
            *count += 1;
            self.k = self.k.max(*count);
        }
        self.k
    }
}

#[cfg(test)]
mod tests {
    use super::MyCalendarThree;

    #[test]
    fn test_case() {
        let mut calender = MyCalendarThree::new();
        assert_eq!(1, calender.book(10, 20));
        assert_eq!(1, calender.book(50, 60));
        assert_eq!(2, calender.book(10, 40));
        assert_eq!(3, calender.book(5, 15));
        assert_eq!(3, calender.book(5, 10));
        assert_eq!(3, calender.book(25, 55));
    }
}
