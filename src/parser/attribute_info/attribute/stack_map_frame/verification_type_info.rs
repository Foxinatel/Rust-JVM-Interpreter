use crate::stream_reader::StreamReader;

#[derive(Debug)]
pub enum VerificationTypeInfo {
  TopVariable,
  IntegerVariable,
  FloatVariable,
  LongVariable,
  DoubleVariable,
  NullVariable,
  UninitializedThisVariable,
  ObjectVariable { cpool_index: u16 },
  UninitializedVariable { offset: u16 }
}

impl VerificationTypeInfo {
  pub fn read(sr: &mut StreamReader) -> Self {
    let tag = sr.get_u8();
    match tag {
      0 => VerificationTypeInfo::TopVariable,
      1 => VerificationTypeInfo::IntegerVariable,
      2 => VerificationTypeInfo::FloatVariable,
      3 => VerificationTypeInfo::DoubleVariable,
      4 => VerificationTypeInfo::LongVariable,
      5 => VerificationTypeInfo::NullVariable,
      6 => VerificationTypeInfo::UninitializedThisVariable,
      7 => VerificationTypeInfo::ObjectVariable { cpool_index: sr.get_u16() },
      8 => VerificationTypeInfo::UninitializedVariable { offset: sr.get_u16() },
      _ => panic!("Invalid VarificationTypeInfo")
    }
  }
}
