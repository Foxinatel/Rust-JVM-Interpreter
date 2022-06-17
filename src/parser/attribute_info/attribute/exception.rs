use crate::stream_reader::StreamReader;

#[derive(Debug)]
pub struct Exception {
  pub start_pc: u16,
  pub end_pc: u16,
  pub handler_pc: u16,
  pub catch_type: u16,
}

impl Exception {
  pub fn read(sr: &mut StreamReader) -> Self {
    return Self {
      start_pc: sr.get_u16(),
      end_pc: sr.get_u16(),
      handler_pc: sr.get_u16(),
      catch_type: sr.get_u16(),
    };
  }
}
