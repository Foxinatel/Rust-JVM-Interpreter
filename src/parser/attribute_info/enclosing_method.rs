use crate::stream_reader::StreamReader;

use super::attribute::ATTRIBUTE;

pub fn read(sr: &mut StreamReader) -> ATTRIBUTE {
  ATTRIBUTE::EnclosingMethod {
    class_index: sr.get_u16(),
    method_index: sr.get_u16(),
  }
}
