use super::attribute::{local_variable::LocalVariable, ATTRIBUTE};
use crate::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> ATTRIBUTE {
  let local_variable_table_length = sr.get_u16();
  let local_variable_table: Vec<LocalVariable> = (0..local_variable_table_length)
    .map(|_| LocalVariable::read(sr))
    .collect();

  ATTRIBUTE::LocalVariableTable {
    local_variable_table
  }
}
