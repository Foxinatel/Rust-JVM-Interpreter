use crate::helpers::get_u16;

use super::attribute::ATTRIBUTE;

pub fn read(buf: &mut &[u8]) -> ATTRIBUTE {
  let number_of_exceptions = get_u16(buf);
  let exception_index_table: Vec<u16> = (0..number_of_exceptions).map(|_| get_u16(buf)).collect();

  ATTRIBUTE::Exceptions {
    number_of_exceptions,
    exception_index_table,
  }
}
