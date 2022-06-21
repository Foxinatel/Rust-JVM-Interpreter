use super::{attribute::local_variable::LocalVariable, Attribute};
use crate::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> Attribute {
  let local_variable_table_length = sr.get_u16();
  let local_variable_table: Vec<LocalVariable> =
    (0..local_variable_table_length).map(|_| LocalVariable::read(sr)).collect();

  Attribute::LocalVariableTable { local_variable_table }
}
