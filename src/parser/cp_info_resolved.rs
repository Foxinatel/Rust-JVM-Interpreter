use super::{attribute_info::attribute::bootstrap_method::BootstrapMethod, cp_info::CpInfo};

#[derive(Debug, Clone)]
pub struct Class {
  pub name: String
}

impl ToString for Class {
  fn to_string(&self) -> String { self.name.to_string() }
}

#[derive(Debug, Clone)]
pub struct NameAndType {
  pub name: String,
  pub descriptor: String
}

impl ToString for NameAndType {
  fn to_string(&self) -> String { format!("{}:{}", self.name, self.descriptor) }
}

#[derive(Debug, Clone)]
pub struct JavaString {
  pub string: String
}

impl ToString for JavaString {
  fn to_string(&self) -> String { self.string.to_string() }
}

#[derive(Debug, Clone)]
pub struct Fieldref {
  pub class: Class,
  pub name_and_type: NameAndType
}

impl ToString for Fieldref {
  fn to_string(&self) -> String {
    format!("{}.{}", self.class.to_string(), self.name_and_type.to_string())
  }
}

#[derive(Debug, Clone)]
pub struct Methodref {
  pub class: Class,
  pub name_and_type: NameAndType
}

impl ToString for Methodref {
  fn to_string(&self) -> String {
    format!("{}.{}", self.class.to_string(), self.name_and_type.to_string())
  }
}

#[derive(Debug, Clone)]
pub struct InterfaceMethodref {
  pub class: Class,
  pub name_and_type: NameAndType
}

#[derive(Debug, Clone)]
pub struct MethodHandle {
  //TODO
}

#[derive(Debug, Clone)]
pub struct MethodType {
  pub descriptor: String
}

#[derive(Debug, Clone)]
pub struct InvokeDynamic {
  pub bootstrap_method_attr: BootstrapMethod,
  pub name_and_type: NameAndType
}

#[derive(Debug, Clone)]
pub enum ResolvedCpInfo {
  Class(Class),
  Fieldref(Fieldref),
  Methodref(Methodref),
  InterfaceMethodref(InterfaceMethodref),
  String(JavaString),
  Integer(i32),
  Float(f32),
  Long(i64),
  Double(f64),
  NameAndType(NameAndType),
  MethodHandle(MethodHandle),
  MethodType(MethodType),
  InvokeDynamic(InvokeDynamic),
  Utf8(String)
}

impl ResolvedCpInfo {
  pub fn from(cpinfo: &CpInfo, constant_pool: &Vec<CpInfo>) -> Self {
    match cpinfo {
      CpInfo::Class { name_index } => {
        let CpInfo::Utf8 { bytes: name } = &constant_pool[*name_index as usize -1] else {panic!()};
        ResolvedCpInfo::Class(Class { name: name.clone() })
      }
      CpInfo::Fieldref { class_index, name_and_type_index } => {
        let CpInfo::Class { name_index } = constant_pool[*class_index as usize -1] else {panic!()};
        let CpInfo::Utf8 { bytes: class_name } = &constant_pool[name_index as usize -1] else {panic!()};

        let CpInfo::NameAndType { name_index, descriptor_index } = constant_pool[*name_and_type_index as usize -1] else {panic!()};
        let CpInfo::Utf8 { bytes: name } = &constant_pool[name_index as usize -1] else {panic!()};
        let CpInfo::Utf8 { bytes: descriptor } = &constant_pool[descriptor_index as usize -1] else {panic!()};

        ResolvedCpInfo::Fieldref(Fieldref {
          class: Class { name: class_name.clone() },
          name_and_type: NameAndType { name: name.clone(), descriptor: descriptor.clone() }
        })
      }
      CpInfo::Methodref { class_index, name_and_type_index } => {
        let CpInfo::Class { name_index } = constant_pool[*class_index as usize -1] else {panic!()};
        let CpInfo::Utf8 { bytes: class_name } = &constant_pool[name_index as usize -1] else {panic!()};

        let CpInfo::NameAndType { name_index, descriptor_index } = constant_pool[*name_and_type_index as usize -1] else {panic!()};
        let CpInfo::Utf8 { bytes: name } = &constant_pool[name_index as usize -1] else {panic!()};
        let CpInfo::Utf8 { bytes: descriptor } = &constant_pool[descriptor_index as usize -1] else {panic!()};

        ResolvedCpInfo::Methodref(Methodref {
          class: Class { name: class_name.clone() },
          name_and_type: NameAndType { name: name.clone(), descriptor: descriptor.clone() }
        })
      }
      CpInfo::InterfaceMethodref { class_index, name_and_type_index } => {
        let CpInfo::Class { name_index } = constant_pool[*class_index as usize -1] else {panic!()};
        let CpInfo::Utf8 { bytes: class_name } = &constant_pool[name_index as usize -1] else {panic!()};

        let CpInfo::NameAndType { name_index, descriptor_index } = constant_pool[*name_and_type_index as usize -1] else {panic!()};
        let CpInfo::Utf8 { bytes: name } = &constant_pool[name_index as usize -1] else {panic!()};
        let CpInfo::Utf8 { bytes: descriptor } = &constant_pool[descriptor_index as usize -1] else {panic!()};

        ResolvedCpInfo::InterfaceMethodref(InterfaceMethodref {
          class: Class { name: class_name.clone() },
          name_and_type: NameAndType { name: name.clone(), descriptor: descriptor.clone() }
        })
      }
      CpInfo::String { string_index } => {
        let CpInfo::Utf8 { bytes: string } = &constant_pool[*string_index as usize -1] else {panic!()};
        ResolvedCpInfo::String(JavaString { string: string.clone() })
      }
      CpInfo::Integer { value } => ResolvedCpInfo::Integer(*value),
      CpInfo::Float { value } => ResolvedCpInfo::Float(*value),
      CpInfo::Long { value } => ResolvedCpInfo::Long(*value),
      CpInfo::Double { value } => ResolvedCpInfo::Double(*value),
      CpInfo::NameAndType { name_index, descriptor_index } => {
        let CpInfo::Utf8 { bytes: name } = &constant_pool[*name_index as usize -1] else {panic!()};
        let CpInfo::Utf8 { bytes: descriptor } = &constant_pool[*descriptor_index as usize -1] else {panic!()};
        ResolvedCpInfo::NameAndType(NameAndType {
          name: name.clone(),
          descriptor: descriptor.clone()
        })
      }
      CpInfo::Utf8 { bytes } => ResolvedCpInfo::Utf8(bytes.to_string()),
      CpInfo::MethodHandle { reference_kind: _, reference_index: _ } => todo!(),
      CpInfo::MethodType { descriptor_index: _ } => todo!(),
      CpInfo::InvokeDynamic { bootstrap_method_attr_index: _, reference_index: _ } => todo!()
    }
  }
}
