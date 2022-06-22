use self::verification_type_info::VerificationTypeInfo;
use crate::parser::stream_reader::StreamReader;

mod verification_type_info;

#[derive(Debug, Clone)]
pub enum StackMapFrame {
  SameFrame {
    frame_type: u8
  },
  SameLocals1StackItemFrame {
    frame_type: u8,
    stack: [VerificationTypeInfo; 1]
  },
  SameLocals1StackItemFrameExtended {
    frame_type: u8,
    offset_delta: u16,
    stack: [VerificationTypeInfo; 1]
  },
  ChopFrame {
    frame_type: u8,
    offset_delta: u16
  },
  SameFrameExtended {
    frame_type: u8,
    offset_delta: u16
  },
  AppendFrame {
    frame_type: u8,
    offset_delta: u16,
    locals: Vec<VerificationTypeInfo>
  },
  FullFrame {
    frame_type: u8,
    offset_delta: u16,
    number_of_locals: u16,
    locals: Vec<VerificationTypeInfo>,
    number_of_stack_items: u16,
    stack: Vec<VerificationTypeInfo>
  }
}

impl StackMapFrame {
  pub fn read(sr: &mut StreamReader) -> Self {
    let frame_type = sr.get_u8();
    match frame_type {
      (0..=63) => StackMapFrame::SameFrame { frame_type },
      (64..=127) => StackMapFrame::SameLocals1StackItemFrame {
        frame_type,
        stack: [VerificationTypeInfo::read(sr)]
      },
      247 => StackMapFrame::SameLocals1StackItemFrameExtended {
        frame_type,
        offset_delta: sr.get_u16(),
        stack: [VerificationTypeInfo::read(sr)]
      },
      (248..=250) => StackMapFrame::ChopFrame { frame_type, offset_delta: sr.get_u16() },
      251 => StackMapFrame::SameFrameExtended { frame_type, offset_delta: sr.get_u16() },
      (252..=254) => {
        let offset_delta = sr.get_u16();
        let locals: Vec<VerificationTypeInfo> =
          (0..frame_type - 251).map(|_| VerificationTypeInfo::read(sr)).collect();
        StackMapFrame::AppendFrame { frame_type, offset_delta, locals }
      }
      255 => {
        let offset_delta = sr.get_u16();
        let number_of_locals = sr.get_u16();
        let locals: Vec<VerificationTypeInfo> =
          (0..number_of_locals).map(|_| VerificationTypeInfo::read(sr)).collect();
        let number_of_stack_items = sr.get_u16();
        let stack: Vec<VerificationTypeInfo> =
          (0..number_of_stack_items).map(|_| VerificationTypeInfo::read(sr)).collect();
        StackMapFrame::FullFrame {
          frame_type,
          offset_delta,
          number_of_locals,
          locals,
          number_of_stack_items,
          stack
        }
      }
      _ => panic!("This tag is reserved for future use")
    }
  }
}
