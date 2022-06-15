use crate::helpers::get_u16;

#[derive(Debug)]
pub struct Classes {
  pub inner_class_info_index: u16,
  pub outer_class_info_index: u16,
  pub inner_name_index: u16,
  pub inner_class_access_flags: u16,
}

impl Classes {
  pub fn read(buf: &mut &[u8]) -> Self {
    Classes {
      inner_class_info_index: get_u16(buf),
      outer_class_info_index: get_u16(buf),
      inner_name_index: get_u16(buf),
      inner_class_access_flags: get_u16(buf),
    }
  }
}
