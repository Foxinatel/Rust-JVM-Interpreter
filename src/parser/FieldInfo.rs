use super::{
  helpers::{get_u16},
  CpInfo::{CpInfo},
  AttributeInfo::AttributeInfo
};

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>
}

impl FieldInfo {
    pub fn read(buf: &mut &[u8], constant_pool: &Vec<CpInfo>) -> Self {
        let access_flags = get_u16(buf);
        let name_index = get_u16(buf);
        let descriptor_index = get_u16(buf);
        let attributes_count = get_u16(buf);
        let attributes: Vec<AttributeInfo> = (0..attributes_count).map(|_|
            AttributeInfo::read(buf, &constant_pool)
        ).collect();

        Self { access_flags, name_index, descriptor_index, attributes_count, attributes}
    }
}