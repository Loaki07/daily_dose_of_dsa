struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable
 * reference. If you need a mutable reference,
 * change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            stack1: vec![],
            stack2: vec![],
        }
    }

    // making push the expensive operation
    // move everything from stack1 to stack 2
    // push the element to stack1
    // and then pop everything from stack 2
    // back to stack 1
    fn push(&mut self, x: i32) {
        while let Some(val) = self.stack1.pop() {
            self.stack2.push(val);
        }
        self.stack1.push(x);
        while let Some(val) = self.stack2.pop() {
            self.stack1.push(val);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack1.pop().unwrap()
    }

    // view the 1st element of the queue
    fn peek(&self) -> i32 {
        self.stack1.last().unwrap().clone()
    }

    fn empty(&self) -> bool {
        self.stack1.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert!(!q.empty());
        assert_eq!(q.pop(), 2);
        assert!(q.empty());
    }
}
