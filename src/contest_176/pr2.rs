struct ProductOfNumbers {
    storage: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        ProductOfNumbers { storage: Vec::new() }
    }
    
    fn add(&mut self, num: i32) {
        self.storage.push(num);
    }
    
    fn get_product(&self, k: i32) -> i32 {
        self.storage.iter().rev().take(k as usize).fold(1, |prod, n| prod * n)
    }
}

// 
// Your ProductOfNumbers object will be instantiated and called as such:
// let obj = ProductOfNumbers::new();
// obj.add(num);
// let ret_2: i32 = obj.get_product(k);
// 