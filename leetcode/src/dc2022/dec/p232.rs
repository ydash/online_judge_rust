// 232. Implement Queue using Stacks

struct MyQueue {
    front: Vec<i32>,
    back: Vec<i32>,
}

impl MyQueue {
    #[allow(dead_code)]
    fn new() -> Self {
        MyQueue {
            front: vec![],
            back: vec![],
        }
    }

    #[allow(dead_code)]
    fn push(&mut self, x: i32) {
        self.back.push(x);
    }

    #[allow(dead_code)]
    fn pre_proc(&mut self) {
        if self.front.is_empty() {
            while let Some(x) = self.back.pop() {
                self.front.push(x);
            }
        }
    }

    #[allow(dead_code)]
    fn pop(&mut self) -> i32 {
        self.pre_proc();
        self.front.pop().unwrap()
    }

    #[allow(dead_code)]
    fn peek(&mut self) -> i32 {
        self.pre_proc();
        *self.front.last().unwrap()
    }

    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::MyQueue;

    #[test]
    fn test_case_01() {
        let mut queue = MyQueue::new();
        assert_eq!(true, queue.empty());
        queue.push(1);
        queue.push(2);
        assert_eq!(1, queue.peek());
        assert_eq!(1, queue.pop());
        assert_eq!(false, queue.empty());
        queue.push(3);
        assert_eq!(2, queue.peek());
        assert_eq!(2, queue.pop());
        assert_eq!(3, queue.pop());
        assert_eq!(true, queue.empty());
    }
}
