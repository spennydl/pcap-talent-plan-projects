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

extern crate failure;
extern crate failure_derive;

use std::io;

use failure::Fail;

mod store;
pub use store::KvStore;

mod log;

/// Error type for the Kvs lib
#[derive(Fail, Debug)]
pub enum KvError {

    /// Io error occurred
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    /// Serialization error
    #[fail(display = "{}", _0)]
    SerializationError(#[cause] Box<bincode::ErrorKind>),

    /// Key Not Found
    #[fail(display = "Key not found")]
    NonExistentKey
}

impl From<io::Error> for KvError {
    fn from(err: io::Error) -> KvError {
        KvError::Io(err)
    }
}

impl From<Box<bincode::ErrorKind>> for KvError {
    fn from(err: Box<bincode::ErrorKind>) -> KvError {
        KvError::SerializationError(err)
    }
}

/// Result type.
pub type Result<T> = std::result::Result<T, KvError>;

