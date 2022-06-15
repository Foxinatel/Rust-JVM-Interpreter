use crate::helpers::get_u16;

use super::attribute::{classes::Classes, ATTRIBUTE};

pub fn read(buf: &mut &[u8]) -> ATTRIBUTE {
  let number_of_classes = get_u16(buf);
  let classes: Vec<Classes> = (0..number_of_classes).map(|_|
    Classes::read(buf)
  ).collect();

  ATTRIBUTE::InnerClasses {
    number_of_classes,
    classes
  }
}
