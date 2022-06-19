#![feature(let_else)]

use std::env;

use crate::evaluate::JVM;

mod evaluate;
mod parser;
mod stream_reader;

fn main() {
  let path = env::args().skip(1).next().expect("Expected File Name");

  // let cf = ClassFile::read(path);
  // println!("{:#?}", cf);

  let jvm = JVM::from_path(path);
  println!("{:#?}", jvm);
}
