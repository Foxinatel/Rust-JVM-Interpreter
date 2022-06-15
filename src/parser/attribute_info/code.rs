use super::{attribute::{ATTRIBUTE, exception::Exception}, AttributeInfo};
use crate::parser::{helpers::{get_u16, get_u32, take_n}, cp_info::CpInfo};

pub fn read(buf: &mut &[u8], constant_pool: &Vec<CpInfo>) -> ATTRIBUTE {
  let max_stack = get_u16(buf);
  let max_locals = get_u16(buf);
  let code_length = get_u32(buf);
  let code = take_n(code_length as usize, buf);
  let exception_table_length = get_u16(buf);
  let exception_table: Vec<Exception> = (0..exception_table_length).map(|_|
      Exception::read(buf)
  ).collect();
  let attributes_count =  get_u16(buf);
  let attributes: Vec<AttributeInfo> = (0..attributes_count).map(|_|
      AttributeInfo::read(buf, constant_pool)
  ).collect();
  ATTRIBUTE::Code { max_stack, max_locals, code_length, code, exception_table_length, exception_table, attributes_count, attributes}
}