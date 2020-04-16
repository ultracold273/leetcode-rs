
struct CustomStack {
    max_size: usize,
    stack: Vec<i32>
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self { max_size: max_size as usize, stack: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        // let n = if self.stack.len() < k as usize { self.stack.len() } else { k as usize };
        let n = (k as usize).min(self.stack.len());
        for i in 0..n {
            self.stack[i] += val;
        }
    }
}