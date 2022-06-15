use crate::parser::helpers::get_u8;

use super::attribute::ATTRIBUTE;

pub fn read(buf: &mut &[u8], attribute_length: u32) -> ATTRIBUTE {
  let debug_extension: Vec<u8> = (0..attribute_length).map(|_|
    get_u8(buf)
  ).collect();
  ATTRIBUTE::SourceDebugExtension { debug_extension }
}