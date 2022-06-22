use crate::parser::stream_reader::StreamReader;

#[derive(Debug, Clone)]
pub struct LineNumber {
  pub start_pc: u16,
  pub line_number: u16
}

impl LineNumber {
  pub fn read(sr: &mut StreamReader) -> Self {
    LineNumber { start_pc: sr.get_u16(), line_number: sr.get_u16() }
  }
}
