use crate::helpers::get_u16;

use super::attribute::{local_variable::LocalVariable, ATTRIBUTE};

pub fn read(buf: &mut &[u8]) -> ATTRIBUTE {
  let local_variable_table_length = get_u16(buf);
  let local_variable_table: Vec<LocalVariable> = (0..local_variable_table_length)
    .map(|_| LocalVariable::read(buf))
    .collect();

  ATTRIBUTE::LocalVariableTable {
    local_variable_table_length,
    local_variable_table,
  }
}
