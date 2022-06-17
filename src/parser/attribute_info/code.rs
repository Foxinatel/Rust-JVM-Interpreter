use self::{code_generator::generate_instructions, clean_jumps::clean};

use super::{
  attribute::{exception::Exception, ATTRIBUTE},
  AttributeInfo,
};
use crate::{
  parser::cp_info::CpInfo,
  stream_reader::StreamReader,
};

pub mod code_generator;
pub mod clean_jumps;

pub fn read(sr: &mut StreamReader, constant_pool: &Vec<CpInfo>) -> ATTRIBUTE {
  let max_stack = sr.get_u16();
  let max_locals = sr.get_u16();
  let code_length = sr.get_u32();
  let raw_code = sr.take_n(code_length as usize);
  let tuple_code = generate_instructions(&mut StreamReader::from(raw_code));
  let code = clean(tuple_code);
  let exception_table_length = sr.get_u16();
  let exception_table: Vec<Exception> = (0..exception_table_length)
    .map(|_| Exception::read(sr))
    .collect();
  let attributes_count = sr.get_u16();
  let attributes: Vec<AttributeInfo> = (0..attributes_count)
    .map(|_| AttributeInfo::read(sr, constant_pool))
    .collect();

  ATTRIBUTE::Code {
    max_stack,
    max_locals,
    code_length,
    code,
    exception_table_length,
    exception_table,
    attributes_count,
    attributes,
  }
}
