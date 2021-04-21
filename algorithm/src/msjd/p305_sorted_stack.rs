struct SortedStack {
    stack: Vec<i32>,
}

impl SortedStack {
    fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        self.stack.sort_by(|e1, e2| e1.cmp(&e2))
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn peek(&self) -> i32 {
        *self.stack.last().unwrap_or(&-1)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[test]
fn test() {}