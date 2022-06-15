use crate::helpers::get_u16;

use super::attribute::{ATTRIBUTE, local_variable_type::LocalVariableType};

pub fn read(buf: &mut &[u8]) -> ATTRIBUTE {
  let local_variable_type_table_length = get_u16(buf);
  let local_variable_type_table: Vec<LocalVariableType> = (0..local_variable_type_table_length).map(|_|
    LocalVariableType::read(buf)
  ).collect();

  ATTRIBUTE::LocalVariableTypeTable {
    local_variable_type_table_length,
    local_variable_type_table
  }
}
