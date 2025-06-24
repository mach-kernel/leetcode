use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct LRUCache {
    pub capacity: usize,
    store: HashMap<i32, i32>,
    q: VecDeque<i32>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            store: HashMap::new(),
            q: VecDeque::with_capacity(capacity as usize)
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(v) = self.store.get(&key) {
            let res = *v;
            self.use_key(key.clone());
            res
        } else {
            -1
        }
    }

    fn use_key(&mut self, key: i32) {
        self.q.retain(|k| *k != key);
        self.q.push_front(key);
    }

    fn put(&mut self, key: i32, value: i32) {
        if (self.store.contains_key(&key) || self.store.len() < self.capacity) {
            self.store.insert(key, value);
            self.use_key(key);
            return;
        }

        if let Some(cya) = self.q.pop_back() {
            self.store.remove(&cya);
        }

        self.put(key, value);
    }
}

#[cfg(test)]
mod tests {
    use crate::p146::LRUCache;

    #[test]
    fn test_lru() {
        let mut lru = LRUCache::new(2);
        lru.put(2,1);
        println!("{:?}", lru);
        lru.put(1,1);
        println!("{:?}", lru);
        lru.put(2,3);
        println!("{:?}", lru);
        lru.put(4,1);
        println!("{:?}", lru);
        lru.get(1);
        lru.get(2);
    }
}