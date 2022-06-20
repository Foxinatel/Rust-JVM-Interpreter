
use crate::stream_reader::StreamReader;

use super::Attribute;

pub fn read(sr: &mut StreamReader) -> Attribute {
  Attribute::ConstantValue { constantvalue_index: sr.get_u16() }
}
