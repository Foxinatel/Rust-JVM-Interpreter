use super::{attribute::line_number::LineNumber, Attribute};
use crate::parser::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> Attribute {
  let line_number_table_length = sr.get_u16();
  let line_number_table: Vec<LineNumber> =
    (0..line_number_table_length).map(|_| LineNumber::read(sr)).collect();

  Attribute::LineNumberTable { line_number_table }
}
