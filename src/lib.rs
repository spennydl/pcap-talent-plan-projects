use std::collections::HashMap;

pub struct KvStore {
    store: HashMap<String, String>
}


impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new()
        }
    }

    pub fn set(&mut self, key: String, val: String) {
        self.store.insert(key, val);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
