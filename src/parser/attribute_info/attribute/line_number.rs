use crate::parser::helpers::get_u16;

#[derive(Debug)]
pub struct LineNumber {
  start_pc: u16,
  line_number: u16
}

impl LineNumber {
  pub fn read(buf: &mut &[u8]) -> Self {
    LineNumber {
      start_pc: get_u16(buf),
      line_number: get_u16(buf)
    }
  }
}