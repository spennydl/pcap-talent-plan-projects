use std::path;

use super::Result;
use crate::log::{KvIndex, KvLog};

/// An in-memory key-value store.  Currently nothing more
/// than a wrapper around a hash map.
pub struct KvStore {
    index: KvIndex,
}

impl KvStore {
    /// Get a new KvStore.
    /// 
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut kvs = KvStore::new();
    /// ```
    pub fn new() -> Result<KvStore> {
        let log = KvLog::open(&std::path::Path::new("kv.db"))?;
        let index = KvIndex::new(log)?;

        Ok(KvStore {
            index
        })
    }

    /// Open a store.
    /// 
    /// ```rust
    /// # use kvs::KvStore;
    /// let path = Path::from("/path/to/store.db");
    /// let mut kvs = KvStore::open(&path)?;
    /// ```
    pub fn open(path: &path::Path) -> Result<KvStore> {
        let path = path.join("kv.db");
        let log = KvLog::open(&path)?;
        let index = KvIndex::new(log)?;

        Ok(KvStore {
            index
        })
    }

    /// Set an item in a store.
    /// ```rust
    /// # use kvs::KvStore;
    /// # let mut kvs = KvStore::new();
    /// kvs.set("my_key".to_string(), "my_value".to_string());
    /// ```
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        self.index.set(key, val)?;
        Ok(())
    }

    /// Get a _copy_ of a value from the store.
    /// ```rust
    /// # use kvs::KvStore;
    /// # let mut kvs = KvStore::new();
    /// if let Some(value) = kvs.get("key".to_string()) {
    ///     // process value
    /// }
    pub fn get(&self, key: String) -> Result<Option<String>> {
        self.index.lookup(&key)
    }

    /// Remove an item from the store.
    /// ```rust
    /// # use kvs::KvStore;
    /// # let mut kvs = KvStore::new();
    /// kvs.remove("key".to_string());
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.index.remove(key)?;
        Ok(())
    }
}
