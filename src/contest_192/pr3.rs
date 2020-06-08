struct BrowserHistory {
    history: Vec<String>,
    visiting: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {

    fn new(homepage: String) -> Self {
        Self { history: vec![homepage], visiting: 0 }
    }
    
    fn visit(&mut self, url: String) {
        if self.history.len() - 1 > self.visiting {
            self.history[self.visiting + 1] = url;
            self.history.resize(self.visiting+2, "".to_string());
        } else {
            self.history.push(url);
        }
        self.visiting += 1;
    }
    
    fn back(&mut self, steps: i32) -> String {
        if self.visiting >= steps as usize {
            self.visiting = self.visiting - steps as usize;
        } else {
            self.visiting = 0;
        }
        self.history[self.visiting].clone()
    }
    
    fn forward(&mut self, steps: i32) -> String {
        if (self.visiting + steps as usize) < self.history.len() {
            self.visiting += steps as usize;
        } else {
            self.visiting = self.history.len() - 1;
        }
        self.history[self.visiting].clone()
    }
}