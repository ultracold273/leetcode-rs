

// Seems the compiler version
use std::collections::HashMap;

struct Cashier {
    current: i32,
    n: i32,
    discount: i32,
    catalog: HashMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {

    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        let nprods = products.len();
        for i in 0..nprods {
            map.insert(products[i], prices[i]);
        }
        Cashier {current: 0, n: n, discount: discount, catalog: map }
    }
    
    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let mut bill = 0;
        let n = product.len();
        for i in 0..n {
            let item = product[i];
            let price = self.catalog.get(&item).unwrap();
            bill += price * amount[i];
        }

        let mut bill = bill as f64;
        
        self.current += 1;
        if self.current == self.n {
            bill = bill - (self.discount as f64 * bill) / 100f64;
            self.current = 0;
        }
        bill
    }
}