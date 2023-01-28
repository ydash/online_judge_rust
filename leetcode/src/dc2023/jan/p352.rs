// 352. Data Stream as Disjoint Intervals

use std::collections::BTreeMap;

struct SummaryRanges {
    intervals: BTreeMap<i32, i32>,
}
impl SummaryRanges {
    #[allow(dead_code)]
    fn new() -> Self {
        SummaryRanges {
            intervals: BTreeMap::new(),
        }
    }

    #[allow(dead_code)]
    fn add_num(&mut self, value: i32) {
        match (
            self.intervals.range(..(value)).last(),
            self.intervals.range((value)..).next(),
        ) {
            (None, None) => {
                self.intervals.insert(value, value);
            }
            (None, Some((&s, &e))) => {
                if s - 1 == value {
                    self.intervals.insert(value, e);
                    self.intervals.remove(&s);
                } else if value < s {
                    self.intervals.insert(value, value);
                }
            }
            (Some((&s, &e)), None) => {
                if e + 1 == value {
                    self.intervals.insert(s, value);
                } else if e < value {
                    self.intervals.insert(value, value);
                }
            }
            (Some((&s1, &e1)), Some((&s2, &e2))) => {
                if e1 + 1 == value && s2 - 1 == value {
                    self.intervals.insert(s1, e2);
                    self.intervals.remove(&s2);
                } else if e1 + 1 == value {
                    self.intervals.insert(s1, value);
                } else if s2 - 1 == value {
                    self.intervals.insert(value, e2);
                    self.intervals.remove(&s2);
                } else if e1 < value && s2 > value {
                    self.intervals.insert(value, value);
                }
            }
        };
    }

    #[allow(dead_code)]
    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals.iter().map(|(&a, &b)| vec![a, b]).collect()
    }
}

#[test]
fn test_summary_ranges() {
    let mut sr = SummaryRanges::new();
    sr.add_num(1);
    assert_eq!(vec![vec![1, 1]], sr.get_intervals());
    sr.add_num(3);
    assert_eq!(vec![vec![1, 1], vec![3, 3]], sr.get_intervals());
    sr.add_num(9);
    assert_eq!(vec![vec![1, 1], vec![3, 3], vec![9, 9]], sr.get_intervals());
    sr.add_num(2);
    assert_eq!(vec![vec![1, 3], vec![9, 9]], sr.get_intervals());
    sr.add_num(8);
    assert_eq!(vec![vec![1, 3], vec![8, 9]], sr.get_intervals());
    sr.add_num(10);
    assert_eq!(vec![vec![1, 3], vec![8, 10]], sr.get_intervals());
    sr.add_num(9);
    assert_eq!(vec![vec![1, 3], vec![8, 10]], sr.get_intervals());
    sr.add_num(2);
    assert_eq!(vec![vec![1, 3], vec![8, 10]], sr.get_intervals());
    sr.add_num(5);
    assert_eq!(
        vec![vec![1, 3], vec![5, 5], vec![8, 10]],
        sr.get_intervals()
    );
    sr.add_num(6);
    assert_eq!(
        vec![vec![1, 3], vec![5, 6], vec![8, 10]],
        sr.get_intervals()
    );
    sr.add_num(1);
    assert_eq!(
        vec![vec![1, 3], vec![5, 6], vec![8, 10]],
        sr.get_intervals()
    );
}
