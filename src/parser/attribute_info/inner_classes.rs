use super::attribute::{classes::Classes, ATTRIBUTE};
use crate::stream_reader::StreamReader;

pub fn read(sr: &mut StreamReader) -> ATTRIBUTE {
  let number_of_classes = sr.get_u16();
  let classes: Vec<Classes> = (0..number_of_classes).map(|_| Classes::read(sr)).collect();

  ATTRIBUTE::InnerClasses { classes }
}
