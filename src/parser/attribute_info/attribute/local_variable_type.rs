use crate::parser::stream_reader::StreamReader;

#[derive(Debug, Clone)]
pub struct LocalVariableType {
  pub start_pc: u16,
  pub length: u16,
  pub name_index: u16,
  pub signature_index: u16,
  pub index: u16
}

impl LocalVariableType {
  pub fn read(sr: &mut StreamReader) -> Self {
    LocalVariableType {
      start_pc: sr.get_u16(),
      length: sr.get_u16(),
      name_index: sr.get_u16(),
      signature_index: sr.get_u16(),
      index: sr.get_u16()
    }
  }
}
