#![feature(let_else)]

use std::env;

use crate::{
  evaluate::{minify::MinifiedClassFile, JVM},
  parser::classfile::ClassFile
};

mod evaluate;
mod parser;
mod stream_reader;

fn main() {
  let path = env::args().skip(1).next().expect("Expected File Name");

  let cf = ClassFile::read(path.clone());
  println!("{:#?}", cf);

  let cf2 = MinifiedClassFile::from(cf.1);
  println!("{:#?}", cf2);

  let jvm = JVM::from_path(path);
  println!("{:#?}", jvm);
}
