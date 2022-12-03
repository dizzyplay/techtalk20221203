use std::error::Error;
use std::fmt::Result;
use std::fs;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");
    let mut file = fs::File::create("/helloworld/hello.txt").unwrap();
    write!(file, "hello world").unwrap();
}
