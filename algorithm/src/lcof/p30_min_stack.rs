struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if x <= self.min() {
            self.min_stack.push(x);
        }
    }

    fn pop(&mut self) {
        if let Some(v) = self.stack.pop() {
            if v == self.min() {
                self.min_stack.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        if let Some(v) = self.stack.last() {
            *v
        } else {
            std::i32::MAX
        }
    }

    fn min(&self) -> i32 {
        if let Some(v) = self.min_stack.last() {
            *v
        } else {
            std::i32::MAX
        }
    }
}
