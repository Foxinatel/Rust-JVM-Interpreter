use super::attribute::{line_number::LineNumber, ATTRIBUTE};
use crate::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> ATTRIBUTE {
  let line_number_table_length = sr.get_u16();
  let line_number_table: Vec<LineNumber> = (0..line_number_table_length)
    .map(|_| LineNumber::read(sr))
    .collect();

  ATTRIBUTE::LineNumberTable {
    line_number_table_length,
    line_number_table
  }
}
