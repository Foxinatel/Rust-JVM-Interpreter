#![feature(let_else)]

use std::env;

use crate::{evaluate::JVM, parser::classfile::ClassFile};

mod evaluate;
mod parser;
mod stream_reader;

fn main() {
  let path = env::args().skip(1).next().expect("Expected File Name");

  // let (name, cf, depends) = ClassFile::read(path.clone());
  // println!("{:#?}\ndepends:{:?}\n", cf, depends);

  let jvm = JVM::from_path(path);
  println!("{:#?}", jvm);
}
