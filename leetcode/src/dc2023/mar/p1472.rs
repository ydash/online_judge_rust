// 1472. Design Browser History

struct BrowserHistory {
    i: usize,
    last: usize,
    cache: Vec<String>,
}

impl BrowserHistory {
    #[allow(dead_code)]
    fn new(homepage: String) -> Self {
        BrowserHistory {
            i: 0,
            last: 0,
            cache: vec![homepage],
        }
    }

    #[allow(dead_code)]
    fn visit(&mut self, url: String) {
        self.i += 1;
        self.last = self.i;
        if self.cache.len() < self.i + 1 {
            self.cache.push(url);
        } else {
            self.cache[self.i] = url;
        }
    }

    #[allow(dead_code)]
    fn back(&mut self, steps: i32) -> String {
        self.i -= self.i.min(steps as usize);
        self.cache[self.i].clone()
    }

    #[allow(dead_code)]
    fn forward(&mut self, steps: i32) -> String {
        self.i = self.last.min(self.i + steps as usize);
        self.cache[self.i].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::BrowserHistory;

    #[test]
    fn test_case_01() {
        let mut browser_history = BrowserHistory::new("leetcode.com".to_owned());
        browser_history.visit("google.com".to_owned());
        browser_history.visit("facebook.com".to_owned());
        browser_history.visit("youtube.com".to_owned());
        assert_eq!("facebook.com", browser_history.back(1));
        assert_eq!("google.com", browser_history.back(1));
        assert_eq!("facebook.com", browser_history.forward(1));
        browser_history.visit("linkedin.com".to_owned());
        assert_eq!("linkedin.com", browser_history.forward(1));
        assert_eq!("google.com", browser_history.back(2));
        assert_eq!("leetcode.com", browser_history.back(7));
    }
}
