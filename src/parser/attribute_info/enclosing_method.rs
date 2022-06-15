use crate::parser::helpers::get_u16;

use super::attribute::ATTRIBUTE;

pub fn read(buf: &mut &[u8]) -> ATTRIBUTE {
  ATTRIBUTE::EnclosingMethod {
    class_index: get_u16(buf),
    method_index: get_u16(buf)
  }
}