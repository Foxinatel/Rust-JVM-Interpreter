use super::attribute::{local_variable_type::LocalVariableType, ATTRIBUTE};
use crate::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> ATTRIBUTE {
  let local_variable_type_table_length = sr.get_u16();
  let local_variable_type_table: Vec<LocalVariableType> = (0..local_variable_type_table_length)
    .map(|_| LocalVariableType::read(sr))
    .collect();

  ATTRIBUTE::LocalVariableTypeTable {
    local_variable_type_table
  }
}
