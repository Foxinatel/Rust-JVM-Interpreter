use crate::{parser::cp_info_resolved::ResolvedCpInfo, stream_reader::StreamReader};

use super::Attribute;

pub fn read(sr: &mut StreamReader, constant_pool: &Vec<ResolvedCpInfo>) -> Attribute {
  let signature_index = sr.get_u16();
  let ResolvedCpInfo::Utf8 (signature) = &constant_pool[signature_index as usize - 1] else {panic!()};

  Attribute::Signature { signature: signature.to_string() }
}
