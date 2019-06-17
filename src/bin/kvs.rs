extern crate structopt;

use kvs::KvStore;
use structopt::StructOpt;

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

fn main() {
    let opt = Kvs::from_args(); 

    match opt {
        Kvs::Get { key } => eprintln!("unimplemented"),
        Kvs::Set { key, value } => eprintln!("unimplemented"),
        Kvs::Remove { key } => eprintln!("unimplemented")
    };

    std::process::exit(1);

}