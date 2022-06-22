use super::Attribute;
use crate::parser::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> Attribute {
  Attribute::EnclosingMethod { class_index: sr.get_u16(), method_index: sr.get_u16() }
}
