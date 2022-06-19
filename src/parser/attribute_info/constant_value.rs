use super::attribute::Attribute;
use crate::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> Attribute {
  Attribute::ConstantValue { constantvalue_index: sr.get_u16() }
}
