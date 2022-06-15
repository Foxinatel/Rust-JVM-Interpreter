use crate::parser::helpers::get_u16;

#[derive(Debug)]
pub struct LocalVariableType {
  start_pc: u16,
  length: u16,
  name_index: u16,
  signature_index: u16,
  index: u16
}

impl LocalVariableType {
  pub fn read(buf: &mut &[u8]) -> Self {
    LocalVariableType {
      start_pc: get_u16(buf),
      length: get_u16(buf),
      name_index: get_u16(buf),
      signature_index: get_u16(buf),
      index: get_u16(buf)
    }
  }
}