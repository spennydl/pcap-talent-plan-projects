#![deny(missing_docs)]
//!
//! Provides facilities for creating a managing an in-memory key value store
//! 
//! ```rust
//! use kvs::KvStore;
//! 
//! fn main() {
//!     // create a new store
//!     let mut kvs = KvStore::new();
//! 
//!     // add some items
//!     kvs.set("here comes".to_string(), "another one".to_string());
//!     kvs.set("here it comes".to_string(), "again".to_string());
//! 
//!     // get some items
//!     if let Some(val) = kvs.get("here comes".to_string()) {
//!         println!("{} another one", val);
//!     }
//! 
//!     // remove something
//!     kvs.remove("here it comes".to_string());
//! 
//! }
//! 

use std::collections::HashMap;

/// An in-memory key-value store.  Currently nothing more
/// than a wrapper around a hash map.
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>
}


impl KvStore {
    /// Get a new KvStore.
    /// 
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut kvs = KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new()
        }
    }

    /// Set an item in a store.
    /// ```rust
    /// # use kvs::KvStore;
    /// # let mut kvs = KvStore::new();
    /// kvs.set("my_key".to_string(), "my_value".to_string());
    /// ```
    pub fn set(&mut self, key: String, val: String) {
        self.store.insert(key, val);
    }

    /// Get a _copy_ of a value from the store.
    /// ```rust
    /// # use kvs::KvStore;
    /// # let mut kvs = KvStore::new();
    /// if let Some(value) = kvs.get("key".to_string()) {
    ///     // process value
    /// }
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// Remove an item from the store.
    /// ```rust
    /// # use kvs::KvStore;
    /// # let mut kvs = KvStore::new();
    /// kvs.remove("key".to_string());
    /// ```
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
