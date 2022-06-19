use super::attribute::Attribute;
use crate::{parser::cp_info_resolved::ResolvedCpInfo, stream_reader::StreamReader};

pub fn read(sr: &mut StreamReader, constant_pool: &Vec<ResolvedCpInfo>) -> Attribute {
  let sourcefile_index = sr.get_u16();
  let ResolvedCpInfo::Utf8 (sourcefile) = &constant_pool[sourcefile_index as usize - 1] else {panic!()};

  Attribute::SourceFile { sourcefile: sourcefile.to_string() }
}
