use crate::stream_reader::StreamReader;

use super::Attribute;

pub fn read(sr: &mut StreamReader) -> Attribute {
  Attribute::EnclosingMethod { class_index: sr.get_u16(), method_index: sr.get_u16() }
}
