extern crate crates_index;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("testidx");
    let i = crates_index::Index::new(path);
    if !i.exists() {
        i.clone().unwrap();
    }
    println!("Hello, world!");
}
