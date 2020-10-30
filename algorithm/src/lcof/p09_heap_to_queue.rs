/// 第一次取时，第一个栈中的数据放入第二个栈
struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            s1: Vec::new(),
            s2: Vec::new(),
        }
    }
    fn push(&mut self, x: i32) {
        self.s1.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.s2.is_empty() {
            while let Some(x) = self.s1.pop() {
                self.s2.push(x);
            }
        }
        self.s2.pop().unwrap_or(-1)
    }

    fn peek(&mut self) -> i32 {
        if self.s2.is_empty() {
            while let Some(x) = self.s1.pop() {
                self.s2.push(x);
            }
        }
        *self.s2.last().unwrap_or(&-1)
    }

    fn empty(&self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }
}
