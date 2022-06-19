use crate::stream_reader::StreamReader;

#[derive(Debug, Clone)]
pub struct Classes {
  pub inner_class_info_index: u16,
  pub outer_class_info_index: u16,
  pub inner_name_index: u16,
  pub inner_class_access_flags: u16
}

impl Classes {
  pub fn read(sr: &mut StreamReader) -> Self {
    Classes {
      inner_class_info_index: sr.get_u16(),
      outer_class_info_index: sr.get_u16(),
      inner_name_index: sr.get_u16(),
      inner_class_access_flags: sr.get_u16()
    }
  }
}
