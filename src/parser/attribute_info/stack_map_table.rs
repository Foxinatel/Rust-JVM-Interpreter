use crate::helpers::get_u16;

use super::attribute::{ATTRIBUTE, stack_map_frame::StackMapFrame};

pub fn read(buf: &mut &[u8]) -> ATTRIBUTE {
  let number_of_entries = get_u16(buf);
  let entries: Vec<StackMapFrame> = (0..number_of_entries).map(|_|
    StackMapFrame::read(buf)
  ).collect();

  ATTRIBUTE::StackMapTable {
    number_of_entries,
    entries
  }
}
