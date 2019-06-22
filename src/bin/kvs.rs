extern crate structopt;

use structopt::StructOpt;

use kvs::{KvStore, Result, KvError};

#[derive(StructOpt)]
enum Kvs {
    #[structopt(name = "get")]
    Get {
        key: String
    },

    #[structopt(name = "set")]
    Set {
        key: String,
        value: String
    },

    #[structopt(name = "rm")]
    Remove {
        key: String,
    }
}

fn main() -> Result<()> {
    let opt = Kvs::from_args(); 

    let mut store = KvStore::new()?;

    match opt {
        Kvs::Get { key } => {
            let val = store.get(key)?;
            println!("{}", val.unwrap_or("Key not found".to_string()));
        },
        Kvs::Set { key, value } => {
            store.set(key, value)?;
        },
        Kvs::Remove { key } => {
            match store.remove(key) {
                Err(KvError::NonExistentKey) => {
                    println!("Key not found");
                    std::process::exit(1);
                }
                other => other?
            }
        }
    };

    Ok(())
}
