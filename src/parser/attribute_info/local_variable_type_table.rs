use super::{attribute::local_variable_type::LocalVariableType, Attribute};
use crate::parser::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> Attribute {
  let local_variable_type_table_length = sr.get_u16();
  let local_variable_type_table: Vec<LocalVariableType> =
    (0..local_variable_type_table_length).map(|_| LocalVariableType::read(sr)).collect();

  Attribute::LocalVariableTypeTable { local_variable_type_table }
}
