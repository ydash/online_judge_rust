// 901. Online Stock Span

#[allow(dead_code)]
struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {
    #[allow(dead_code)]
    fn new() -> Self {
        Self { stack: vec![] }
    }

    #[allow(dead_code)]
    fn next(&mut self, price: i32) -> i32 {
        let mut count = 1;
        while !self.stack.is_empty() && self.stack.last().unwrap().0 <= price {
            let (_, c) = self.stack.pop().unwrap();
            count += c;
        }
        self.stack.push((price, count));
        self.stack.last().unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::StockSpanner;

    #[test]
    fn test_case_01() {
        let mut stock_spanner = StockSpanner::new();
        assert_eq!(1, stock_spanner.next(100));
        assert_eq!(1, stock_spanner.next(80));
        assert_eq!(1, stock_spanner.next(60));
        assert_eq!(2, stock_spanner.next(70));
        assert_eq!(1, stock_spanner.next(60));
        assert_eq!(4, stock_spanner.next(75));
        assert_eq!(6, stock_spanner.next(85));
    }
}
