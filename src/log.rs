extern crate serde;
extern crate bincode;
extern crate memmap;

use serde::{Serialize, Deserialize};

use std::fs::{OpenOptions, File};
use std::collections::HashMap;
use std::iter::Iterator;
use memmap::{MmapOptions, Mmap};

use super::{Result, KvError};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum LogCommand {
    Set(String, String),
    Remove(String),
}

pub struct KvLog {
    log_file: File,
    log_map: Mmap,
    log_end: usize
}

impl KvLog {
    pub fn open(path: &std::path::Path) -> Result<KvLog> {
        let log_file = 
            match OpenOptions::new()
            .read(true)
            .write(true)
            .open(path) {
                Ok(file) => file,
                Err(_) => {
                    let file = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .open(path)?;
                    // TODO: faster way to do this?
                    // we don't need to zero it out.
                    //
                    // start with 2 pages, we will try to compact to 1
                    file.set_len(8 * 1024 * 1024)?;
                    file
                }
            };


        let log_map = unsafe {
            MmapOptions::new()
                .map(&log_file)?
        };

        Ok(KvLog {
            log_file,
            log_map,
            log_end: 0,
        })
    }

    pub fn append(&mut self, command: LogCommand) -> Result<usize> {
        let mut mmap = unsafe {
            MmapOptions::new().offset(self.log_end).map_mut(&self.log_file)?
        };

        let encoded: Vec<u8> = bincode::serialize(&command)?;
        mmap.iter_mut().zip(encoded.iter()).for_each(|(dest, src)| *dest = *src);

        let ptr = self.log_end;
        self.log_end = encoded.len();

        Ok(ptr)
    }

    pub fn read(&self, ptr: usize) -> Result<LogCommand> {
        let command: LogCommand = bincode::deserialize(&self.log_map[ptr..])?;
        Ok(command)
    }

    pub fn iter(&self) -> Result<LogIterator> {
        let mmap = unsafe {
            MmapOptions::new().map(&self.log_file)?
        };

        Ok(LogIterator::new(mmap))
    }
}

pub struct LogIterator {
    mmap: Mmap,
    position: usize,
}

impl LogIterator {
    pub fn new(mmap: Mmap) -> LogIterator {
        LogIterator { mmap, position: 0 }
    }
}

impl Iterator for LogIterator {
    type Item = (LogCommand, usize);

    fn next(&mut self) -> Option<(LogCommand, usize)> {
        if self.position >= self.mmap.len() {
            return None;
        }
        
        let command: Option<LogCommand> = bincode::deserialize(&self.mmap[self.position..]).ok();
        let ptr: usize;
        if let Some(cmd) = command {
            ptr = self.position;
            self.position += bincode::serialized_size(&cmd).unwrap_or(0) as usize;

            return Some((cmd, ptr));
        }
        None
    }
}

pub struct KvIndex {
    index: HashMap<String, usize>,
    log: KvLog
}

impl KvIndex {

    pub fn new(log: KvLog) -> Result<KvIndex> {
        let mut index: HashMap<String, usize> = HashMap::new();
        for (command, ptr) in log.iter()? {
            match command {
                LogCommand::Set(key, _) => index.insert(key, ptr),
                LogCommand::Remove(key) => index.remove(&key),
            };
        }

        Ok(KvIndex{ index, log })
    }

    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        let ptr = self.log.append(LogCommand::Set(key.clone(), val.clone()))?;
        self.index.insert(key, ptr);
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.index.remove(&key).ok_or(KvError::NonExistentKey)?;
        self.log.append(LogCommand::Remove(key))?;
        Ok(())
    }

    pub fn lookup(&self, key: &str) -> Result<Option<String>> {
        if let Some(ptr) = self.index.get(key) {
            return match self.log.read(*ptr)? {
                LogCommand::Set(_, value) => {
                    Ok(Some(value))
                },
                _ => {
                    Ok(None)
                }
            }
        }
        Ok(None)
    }

}
