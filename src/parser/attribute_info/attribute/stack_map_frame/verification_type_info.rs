use crate::stream_reader::StreamReader;

#[derive(Debug, Clone)]
pub enum VerificationTypeInfo {
  TopVariable { tag: u8 },
  IntegerVariable { tag: u8 },
  FloatVariable { tag: u8 },
  LongVariable { tag: u8 },
  DoubleVariable { tag: u8 },
  NullVariable { tag: u8 },
  UninitializedThisVariable { tag: u8 },
  ObjectVariable { tag: u8, cpool_index: u16 },
  UninitializedVariable { tag: u8, offset: u16 }
}

impl VerificationTypeInfo {
  pub fn read(sr: &mut StreamReader) -> Self {
    let tag = sr.get_u8();
    match tag {
      0 => VerificationTypeInfo::TopVariable { tag },
      1 => VerificationTypeInfo::IntegerVariable { tag },
      2 => VerificationTypeInfo::FloatVariable { tag },
      3 => VerificationTypeInfo::DoubleVariable { tag },
      4 => VerificationTypeInfo::LongVariable { tag },
      5 => VerificationTypeInfo::NullVariable { tag },
      6 => VerificationTypeInfo::UninitializedThisVariable { tag },
      7 => VerificationTypeInfo::ObjectVariable {
        tag,
        cpool_index: sr.get_u16()
      },
      8 => VerificationTypeInfo::UninitializedVariable {
        tag,
        offset: sr.get_u16()
      },
      _ => panic!("Invalid VarificationTypeInfo")
    }
  }
}
