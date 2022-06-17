use crate::helpers::get_u16;

use super::attribute::{line_number::LineNumber, ATTRIBUTE};

pub fn read(buf: &mut &[u8]) -> ATTRIBUTE {
  let line_number_table_length = get_u16(buf);
  let line_number_table: Vec<LineNumber> = (0..line_number_table_length)
    .map(|_| LineNumber::read(buf))
    .collect();

  ATTRIBUTE::LineNumberTable {
    line_number_table_length,
    line_number_table,
  }
}
