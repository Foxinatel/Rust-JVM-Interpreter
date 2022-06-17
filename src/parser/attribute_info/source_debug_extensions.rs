use crate::helpers::take_n;

use super::attribute::ATTRIBUTE;

pub fn read(buf: &mut &[u8], attribute_length: u32) -> ATTRIBUTE {
  ATTRIBUTE::SourceDebugExtension {
    debug_extension: take_n(attribute_length as usize, buf),
  }
}
