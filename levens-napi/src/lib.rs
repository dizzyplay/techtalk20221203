#![deny(clippy::all)]
mod levenshtein;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn leven(a: String, b: String) -> u8 {
  let mut s = levenshtein::Levenshtein::new(a, b);
  s.distance()
}

#[napi]
pub fn write_file() {
  let mut file = File::create("hello.txt").unwrap();
  write!(file, "hello?").unwrap();
}

#[napi]
pub fn read_file() {
  let mut file = File::open("hello.txt").unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();
  println!("{}", content);
}
