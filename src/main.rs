#![feature(split_array)]
#![feature(slice_take)]

use std::env;
use std::fs;

use crate::parser::ClassFile::ClassFile;

mod parser {
    pub mod ClassFile;
    pub mod AttributeInfo;
    pub mod Classes;
    pub mod CpInfo;
    pub mod ExceptionTable;
    pub mod FieldInfo;
    pub mod helpers;
    pub mod MethodInfo;
    pub mod StackMapFrame;
    pub mod VerificationTypeInfo;
}

fn main() {
    let path = env::args().skip(1).next().expect("Expected File Name");
    let mut buf = &fs::read(path.clone()).or(fs::read(path.clone() + ".class")).expect(
        format!("Could not find a file at {0} or {0}.class",path).as_str()
    )[..];
    let cf = ClassFile::read(&mut buf);
    println!("{:#?}", cf);
}