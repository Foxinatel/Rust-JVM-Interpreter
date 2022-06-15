use crate::helpers::get_u16;

use super::attribute::ATTRIBUTE;

pub fn read(buf: &mut &[u8]) -> ATTRIBUTE {
  ATTRIBUTE::Signature {
    signature_index: get_u16(buf)
  }
}
