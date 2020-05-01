use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, VecDeque};

struct LFUCache {
    size: i32,
    capacity: i32,
    key_map: HashMap<i32, Rc<RefCell<i32>>>,
    freq_map: BTreeMap<i32, VecDeque<Rc<RefCell<i32>>>>,
}

impl LFUCache {

    fn new(capacity: i32) -> Self {
        LFUCache { 
            size: 0,
            capacity: capacity, 
            key_map: HashMap::new(), 
            freq_map: BTreeMap::new() 
        }
    }
    
    fn get(&self, key: i32) -> i32 {
        if let Some(pval) = self.key_map.get(&key) {
            *pval.borrow()
        } else { -1 }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(pval) = self.key_map.get(&key) {
            *pval.borrow_mut() = value;
        } else if self.size + 1 < self.capacity {
            let item = Rc::new(RefCell::new(value));
            self.key_map.insert(key, item);
            let list = self.freq_map.get_mut(&1);

        } else {
        }
    }
}