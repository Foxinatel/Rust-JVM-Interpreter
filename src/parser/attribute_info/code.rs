use std::collections::BTreeMap;

use self::{
  clean_jumps::clean,
  code_generator::{generate_instructions, Instructions}
};
use super::{attribute::exception::Exception, Attribute};
use crate::{parser::cp_info_resolved::ResolvedCpInfo, stream_reader::StreamReader};

pub mod clean_jumps;
pub mod code_generator;

pub fn read(sr: &mut StreamReader, constant_pool: &Vec<ResolvedCpInfo>) -> Attribute {
  let max_stack = sr.get_u16();
  let max_locals = sr.get_u16();
  let code_length = sr.get_u32();
  let raw_code = sr.take_n(code_length as usize);
  let tuple_code: BTreeMap<usize, (usize, Instructions)> =
    generate_instructions(&mut StreamReader::from(raw_code), constant_pool).into_iter().collect();
  let code = clean(tuple_code);
  let exception_table_length = sr.get_u16();
  let exception_table: Vec<Exception> =
    (0..exception_table_length).map(|_| Exception::read(sr)).collect();
  let attributes_count = sr.get_u16();
  let attributes: Vec<Attribute> =
    (0..attributes_count).map(|_| Attribute::read(sr, constant_pool)).collect();

  Attribute::Code { max_stack, max_locals, code, exception_table, attributes }
}
