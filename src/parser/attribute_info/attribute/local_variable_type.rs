use crate::helpers::get_u16;

#[derive(Debug)]
pub struct LocalVariableType {
  pub start_pc: u16,
  pub length: u16,
  pub name_index: u16,
  pub signature_index: u16,
  pub index: u16,
}

impl LocalVariableType {
  pub fn read(buf: &mut &[u8]) -> Self {
    LocalVariableType {
      start_pc: get_u16(buf),
      length: get_u16(buf),
      name_index: get_u16(buf),
      signature_index: get_u16(buf),
      index: get_u16(buf),
    }
  }
}
