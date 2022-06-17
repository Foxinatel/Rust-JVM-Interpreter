use crate::stream_reader::StreamReader;

use super::attribute::{stack_map_frame::StackMapFrame, ATTRIBUTE};

pub fn read(sr: &mut StreamReader) -> ATTRIBUTE {
  let number_of_entries = sr.get_u16();
  let entries: Vec<StackMapFrame> = (0..number_of_entries)
    .map(|_| StackMapFrame::read(sr))
    .collect();

  ATTRIBUTE::StackMapTable {
    number_of_entries,
    entries,
  }
}
