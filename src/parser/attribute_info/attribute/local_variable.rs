use crate::helpers::get_u16;

#[derive(Debug)]
pub struct LocalVariable {
  pub start_pc: u16,
  pub length: u16,
  pub name_index: u16,
  pub descriptor_index: u16,
  pub index: u16
}

impl LocalVariable {
  pub fn read(buf: &mut &[u8]) -> Self {
    LocalVariable {
      start_pc: get_u16(buf),
      length: get_u16(buf),
      name_index: get_u16(buf),
      descriptor_index: get_u16(buf),
      index: get_u16(buf)
    }
  }
}
