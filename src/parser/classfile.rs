use crate::helpers::get_u16;

use super::{
  cp_info::CpInfo,
  field_info::FieldInfo,
  method_info::MethodInfo,
  attribute_info::AttributeInfo,
};

#[derive(Debug)]
pub struct ClassFile {
  pub minor_version: u16,
  pub major_version: u16,
  pub constant_pool_count: u16,
  pub constant_pool: Vec<CpInfo>, //cp_info constant_pool[constant_pool_count-1];
  pub access_flags: u16,
  pub this_class: u16,
  pub super_class: u16,
  pub interfaces_count: u16,
  pub interfaces: Vec<u16>, //u16 interfaces[interfaces_count]
  pub fields_count: u16,
  pub fields: Vec<FieldInfo>, //field_info fields[fields_count];
  pub methods_count: u16,
  pub methods: Vec<MethodInfo>,//method_info methods[methods_count];
  pub attributes_count: u16,
  pub attributes: Vec<AttributeInfo>//attribute_info attributes[attributes_count];
}

impl ClassFile {
  pub fn read(buf: &mut &[u8]) -> Self {
    *buf = buf.strip_prefix(&[0xca, 0xfe, 0xba, 0xbe]).expect("File has invalid header");
    let minor_version = get_u16(buf);
    let major_version = get_u16(buf);
    let constant_pool_count = get_u16(buf);
    let constant_pool: Vec<CpInfo> = (1..constant_pool_count).map(|_| {
        CpInfo::read(buf)
    }).collect();
    let access_flags = get_u16(buf);
    let this_class = get_u16(buf);
    let super_class = get_u16(buf);
    let interfaces_count = get_u16(buf);
    let interfaces: Vec<u16> = (0..interfaces_count).map(|_|
        get_u16(buf)
    ).collect();
    let fields_count = get_u16(buf);
    let fields: Vec<FieldInfo> = (0..fields_count).map(|_|
        FieldInfo::read(buf, &constant_pool)
    ).collect();
    let methods_count = get_u16(buf);
    let methods: Vec<MethodInfo> = (0..methods_count).map(|_|
        MethodInfo::read(buf, &constant_pool)
    ).collect();
    let attributes_count = get_u16(buf);
    let attributes: Vec<AttributeInfo> = (0..attributes_count).map(|_|
        AttributeInfo::read(buf, &constant_pool)
    ).collect();

    return Self {
      minor_version,
      major_version,
      constant_pool_count,
      constant_pool,
      access_flags,
      this_class,
      super_class,
      interfaces_count,
      interfaces,
      fields_count,
      fields,
      methods_count,
      methods,
      attributes_count,
      attributes
    }
  }
}