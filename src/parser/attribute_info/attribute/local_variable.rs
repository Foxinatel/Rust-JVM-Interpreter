use crate::stream_reader::StreamReader;

#[derive(Debug)]
pub struct LocalVariable {
  pub start_pc: u16,
  pub length: u16,
  pub name_index: u16,
  pub descriptor_index: u16,
  pub index: u16
}

impl LocalVariable {
  pub fn read(sr: &mut StreamReader) -> Self {
    LocalVariable {
      start_pc: sr.get_u16(),
      length: sr.get_u16(),
      name_index: sr.get_u16(),
      descriptor_index: sr.get_u16(),
      index: sr.get_u16()
    }
  }
}
