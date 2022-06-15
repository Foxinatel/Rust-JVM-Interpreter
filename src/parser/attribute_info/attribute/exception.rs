use crate::helpers::get_u16;

#[derive(Debug)]
pub struct Exception {
  pub start_pc: u16,
  pub end_pc: u16,
  pub handler_pc: u16,
  pub catch_type: u16,
}

impl Exception {
  pub fn read(buf: &mut &[u8]) -> Self {
    return Self {
      start_pc: get_u16(buf),
      end_pc: get_u16(buf),
      handler_pc: get_u16(buf),
      catch_type: get_u16(buf)
    }
  }
}
