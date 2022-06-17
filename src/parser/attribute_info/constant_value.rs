use super::attribute::ATTRIBUTE;
use crate::helpers::get_u16;

pub fn read(buf: &mut &[u8]) -> ATTRIBUTE {
  ATTRIBUTE::ConstantValue {
    constantvalue_index: get_u16(buf),
  }
}
