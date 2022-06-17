#![feature(let_else)]

use std::{env, fs};

use crate::parser::attribute_info::attribute::ATTRIBUTE;
use crate::parser::attribute_info::code::code_generator::Instructions;
use crate::parser::classfile::ClassFile;
use crate::stream_reader::StreamReader;

mod evaluate;
mod parser;
mod stream_reader;

fn main() {
  let path = env::args().skip(1).next().expect("Expected File Name");
  let buf = fs::read(path.clone()).or(fs::read(path.clone() + ".class")).expect(
    format!("Could not find a file at {0} or {0}.class", path).as_str()
  );
  let mut sr = StreamReader::from(buf);

  let cf = ClassFile::read(&mut sr);
  println!("{:#?}", cf);

  let jvm = evaluate::JVM {
    constant_pool: cf.constant_pool,
  };
  let attribute = ATTRIBUTE::Code {
    max_stack: 2,
    max_locals: 0,
    code_length: 2,
    code: vec![
      Instructions::iconst { value: 5 },
      Instructions::iconst { value: 5 },
      Instructions::iadd,
      Instructions::dup,
      Instructions::iconst { value: 100 },
      Instructions::if_icmplt{ offset: -4 },
      Instructions::ireturn
    ],
    exception_table_length: 0,
    exception_table: vec![],
    attributes_count: 0,
    attributes: vec![],
  };

  println!("{:#?}",jvm.evaluate(attribute));
}