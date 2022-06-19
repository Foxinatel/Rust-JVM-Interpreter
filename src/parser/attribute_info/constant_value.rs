use super::attribute::ATTRIBUTE;
use crate::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> ATTRIBUTE {
  ATTRIBUTE::ConstantValue {
    constantvalue_index: sr.get_u16()
  }
}
