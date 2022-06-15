use super::helpers::{get_u8, get_u16, get_u32, take_n};

#[derive(Debug)]
pub enum CONSTANT {
    Class {tag: u8, name_index: u16},
    Fieldref {tag: u8, class_index: u16, name_and_type_index: u16},
    Methodref {tag: u8, class_index: u16, name_and_type_index: u16},
    InterfaceMethodref {tag: u8, class_index: u16, name_and_type_index: u16},
    String {tag: u8, string_index: u16},
    Integer {tag: u8, bytes: u32},
    Float {tag: u8, bytes: u32},
    Long {tag: u8, high_bytes: u32, low_bytes: u32},
    Double {tag: u8, high_bytes: u32, low_bytes: u32},
    NameAndType {tag: u8, class_index: u16, descriptor_index: u16},
    Utf8 {tag: u8, length: u16, bytes: String},
    MethodHandle {tag: u8, reference_kind: u8, reference_index: u16},
    MethodType {tag: u8, descriptor_index: u16},
    InvokeDynamic {tag: u8, bootstrap_method_attr_index: u16, reference_index: u16},
}

#[derive(Debug)]
pub struct CpInfo {
    tag: u8,
    pub info: CONSTANT
}

impl CpInfo {
    pub fn read(buf: &mut &[u8]) -> Self {
        let tag = get_u8(buf);
        match tag {
            7 => Self { tag, info: CONSTANT::Class { tag, name_index: get_u16(buf) }},
            9 => Self { tag, info: CONSTANT::Fieldref { tag, class_index: get_u16(buf), name_and_type_index: get_u16(buf) }},
            10 => Self { tag, info: CONSTANT::Methodref { tag, class_index: get_u16(buf), name_and_type_index: get_u16(buf) } },
            11 => Self { tag, info: CONSTANT::InterfaceMethodref { tag, class_index: get_u16(buf), name_and_type_index: get_u16(buf) } },
            8 => Self { tag, info: CONSTANT::String { tag, string_index: get_u16(buf) } },
            3 => Self { tag, info: CONSTANT::Integer { tag, bytes: get_u32(buf) } },
            4 => Self { tag, info: CONSTANT::Float { tag, bytes: get_u32(buf) } },
            5 => Self { tag, info: CONSTANT::Long { tag, high_bytes: get_u32(buf), low_bytes: get_u32(buf) } },
            6 => Self { tag, info: CONSTANT::Double { tag, high_bytes: get_u32(buf), low_bytes: get_u32(buf) } },
            12 => Self { tag, info: CONSTANT::NameAndType { tag, class_index: get_u16(buf), descriptor_index: get_u16(buf) } },
            1 => {
                let length = get_u16(buf);
                Self { tag, info: CONSTANT::Utf8 { tag, length, bytes: String::from_utf8(take_n(length as usize, buf)).unwrap() } }
            },
            15 => Self { tag, info: CONSTANT::MethodHandle { tag, reference_kind: get_u8(buf), reference_index: get_u16(buf) } },
            16 => Self { tag, info: CONSTANT::MethodType { tag, descriptor_index: get_u16(buf) } },
            18 => Self { tag, info: CONSTANT::InvokeDynamic { tag, bootstrap_method_attr_index: get_u16(buf), reference_index: get_u16(buf) } },
            other => { eprintln!("ERROR, value was {}", other); panic!() }
        }
    }
}