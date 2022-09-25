// 622. Design Circular Queue

struct MyCircularQueue {
    size: usize,
    first: usize,
    last: i32,
    len: usize,
    content: Vec<i32>,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let k = k as usize;
        MyCircularQueue {
            size: k,
            first: 0,
            last: -1,
            len: 0,
            content: vec![0; k],
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.last = (self.last + 1) % self.size as i32;
            self.content[self.last as usize] = value;
            self.len += 1;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.first = (self.first + 1) % self.size;
            self.len -= 1;
            true
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.content[self.first]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.content[self.last as usize]
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.size
    }
}

#[cfg(test)]
mod tests {
    use super::MyCircularQueue;

    #[test]
    fn test_case_01() {
        let mut q = MyCircularQueue::new(3);
        assert!(q.en_queue(1));
        assert!(q.en_queue(2));
        assert!(q.en_queue(3));
        assert!(!q.en_queue(4));
        assert_eq!(3, q.rear());
        assert!(q.is_full());
        assert_eq!(1, q.front());
        assert!(q.de_queue());
        assert!(q.en_queue(4));
        assert_eq!(4, q.rear());
        assert_eq!(2, q.front());
    }
}
