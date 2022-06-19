use std::collections::HashMap;

use crate::parser::{
  attribute_info::{
    attribute::{
      annotation::Annotation,
      bootstrap_method::BootstrapMethod,
      classes::Classes,
      exception::Exception,
      line_number::LineNumber,
      local_variable::LocalVariable,
      local_variable_type::LocalVariableType,
      parameter_annotation::ParameterAnnotation,
      stack_map_frame::StackMapFrame,
      ATTRIBUTE
    },
    code::code_generator::Instructions,
    AttributeInfo
  },
  classfile::ClassFile,
  cp_info::CpInfo,
  field_info::FieldInfo
};

#[derive(Debug)]
pub enum MiniAttribute {
  ConstantValue {
    constantvalue_index: u16
  },
  Code {
    max_stack: u16,
    max_locals: u16,
    code: Vec<Instructions>,
    exception_table: Vec<Exception>,
    attributes: Vec<AttributeInfo>
  },
  StackMapTable {
    entries: Vec<StackMapFrame>
  },
  Exceptions {
    exception_table: Vec<MiniClassInfo>
  },
  InnerClasses {
    classes: Vec<Classes>
  },
  EnclosingMethod {
    class_info: MiniClassInfo,
    method_info: NameAndTypeInfo
  },
  Synthetic,
  Signature {
    signature: String
  },
  SourceFile {
    sourcefile: String
  },
  SourceDebugExtension {
    debug_extension: Vec<u8>
  },
  LineNumberTable {
    line_number_table: Vec<LineNumber>
  },
  LocalVariableTable {
    local_variable_table: Vec<LocalVariable>
  },
  LocalVariableTypeTable {
    local_variable_type_table: Vec<LocalVariableType>
  },
  Deprecated,
  RuntimeVisibleAnnotations {
    annotations: Vec<Annotation>
  },
  RuntimeInvisibleAnnotations {
    annotations: Vec<Annotation>
  },
  RuntimeVisibleParameterAnnotations {
    parameter_annotations: Vec<ParameterAnnotation>
  },
  RuntimeInvisibleParameterAnnotations {
    parameter_annotations: Vec<ParameterAnnotation>
  },
  AnnotationDefault {
    //TODO
  },
  BootstrapMethods {
    bootstrap_methods: Vec<BootstrapMethod>
  }
}

impl MiniAttribute {
  pub fn from(ai: &AttributeInfo, constant_pool: &Vec<CpInfo>) -> Self {
    match &ai.attribute {
        ATTRIBUTE::ConstantValue { constantvalue_index } => todo!(),
        ATTRIBUTE::Code { max_stack, max_locals, code_length: _, code, exception_table_length, exception_table, attributes_count, attributes } => {
          MiniAttribute::Code { max_stack: *max_stack, max_locals: *max_locals, code: code.to_vec(), exception_table: exception_table.to_vec(), attributes: attributes.to_vec() }
        },
        ATTRIBUTE::StackMapTable { number_of_entries, entries } => {
          MiniAttribute::StackMapTable { entries: entries.to_vec() }
        },
        ATTRIBUTE::Exceptions { number_of_exceptions, exception_index_table } => {
          MiniAttribute::Exceptions { exception_table: exception_index_table.iter().map(|class_index| {
            let CpInfo::Class { tag:_, name_index: index } = constant_pool[*class_index as usize - 1] else { panic!() };
            let CpInfo::Utf8 { tag:_, length:_, bytes: name } = &constant_pool[index as usize - 1] else { panic!() };
            MiniClassInfo{ name: name.to_string() }
          }).collect()}
        },
        ATTRIBUTE::InnerClasses { number_of_classes, classes } => {
          MiniAttribute::InnerClasses { classes: classes.to_vec() }
        },
        ATTRIBUTE::EnclosingMethod { class_index, method_index } => {
          let CpInfo::Class { tag:_, name_index: index } = constant_pool[*class_index as usize - 1] else { panic!() };
          let CpInfo::Utf8 { tag:_, length:_, bytes: class_name1 } = &constant_pool[index as usize - 1] else { panic!() };

          let CpInfo::NameAndType { tag:_, class_index, descriptor_index } = constant_pool[*method_index as usize - 1] else { panic!() };
          let CpInfo::Utf8 { tag:_, length:_, bytes: class_name2 } = &constant_pool[class_index as usize - 1] else { panic!() };
          let CpInfo::Utf8 { tag:_, length:_, bytes: descriptor } = &constant_pool[descriptor_index as usize - 1] else { panic!() };

          MiniAttribute::EnclosingMethod {class_info: MiniClassInfo { name: class_name1.to_string() }, method_info: NameAndTypeInfo{ name: class_name2.to_string(), descriptor: descriptor.to_string() } }
        },
        ATTRIBUTE::Synthetic => MiniAttribute::Synthetic,
        ATTRIBUTE::Signature { signature_index } => {
          let CpInfo::Utf8 { tag:_, length:_, bytes: signature } = &constant_pool[*signature_index as usize - 1] else { panic!() };
          MiniAttribute::Signature { signature: signature.to_string() }
        },
        ATTRIBUTE::SourceFile { sourcefile_index } => {
          let CpInfo::Utf8 { tag:_, length:_, bytes: sourcefile } = &constant_pool[*sourcefile_index as usize - 1] else { panic!() };
          MiniAttribute::SourceFile { sourcefile: sourcefile.to_string() }
        },
        ATTRIBUTE::SourceDebugExtension { debug_extension } => {
          MiniAttribute::SourceDebugExtension { debug_extension: debug_extension.to_vec() }
        },
        ATTRIBUTE::LineNumberTable { line_number_table_length, line_number_table } => {
          MiniAttribute::LineNumberTable { line_number_table: line_number_table.to_vec() }
        },
        ATTRIBUTE::LocalVariableTable { local_variable_table_length, local_variable_table } => {
          MiniAttribute::LocalVariableTable { local_variable_table: local_variable_table.to_vec() }
        },
        ATTRIBUTE::LocalVariableTypeTable { local_variable_type_table_length, local_variable_type_table } => {
          MiniAttribute::LocalVariableTypeTable { local_variable_type_table: local_variable_type_table.to_vec() }
        },
        ATTRIBUTE::Deprecated => MiniAttribute::Deprecated,
        ATTRIBUTE::RuntimeVisibleAnnotations { num_annotations, annotations } => {
          MiniAttribute::RuntimeVisibleAnnotations { annotations: annotations.to_vec() }
        },
        ATTRIBUTE::RuntimeInvisibleAnnotations { num_annotations, annotations } => {
          MiniAttribute::RuntimeInvisibleAnnotations { annotations: annotations.to_vec() }
        },
        ATTRIBUTE::RuntimeVisibleParameterAnnotations { num_annotations, parameter_annotations } => {
          MiniAttribute::RuntimeVisibleParameterAnnotations { parameter_annotations: parameter_annotations.to_vec() }
        },
        ATTRIBUTE::RuntimeInvisibleParameterAnnotations { num_annotations, parameter_annotations } => {
          MiniAttribute::RuntimeInvisibleParameterAnnotations { parameter_annotations: parameter_annotations.to_vec() }
        },
        ATTRIBUTE::AnnotationDefault { attribute_name_index, attribute_length, default_value } => todo!(),
        ATTRIBUTE::BootstrapMethods { num_bootstrap_methods, bootstrap_methods } => {
          MiniAttribute::BootstrapMethods { bootstrap_methods: bootstrap_methods.to_vec() }
        }
    }
  }
}

#[derive(Debug)]
pub struct MiniClassInfo {
  name: String
}

#[derive(Debug)]
pub struct NameAndTypeInfo {
  name: String,
  descriptor: String
}

#[derive(Debug)]
pub struct MiniFieldInfo {
  pub access_flags: u16,
  pub attributes: Vec<MiniAttribute>
}

impl MiniFieldInfo {
  pub fn from(fi: &FieldInfo, constant_pool: &Vec<CpInfo>) -> (String, Self) {
    let access_flags = fi.access_flags;

    let attributes = fi
      .attributes
      .iter()
      .map(|val| MiniAttribute::from(val, &constant_pool))
      .collect();

    let CpInfo::Utf8 { tag:_, length:_, bytes: name } = &constant_pool[fi.name_index as usize - 1] else { panic!() };
    let CpInfo::Utf8 { tag:_, length:_, bytes: descriptor } = &constant_pool[fi.descriptor_index as usize - 1] else { panic!() };
    (name.to_string() + descriptor, Self {
      access_flags,
      attributes
    })
  }
}

#[derive(Debug)]
pub struct MiniMethodInfo {
  pub access_flags: u16,
  pub attributes: Vec<MiniAttribute>
}

impl MiniMethodInfo {
  pub fn from(fi: &FieldInfo, constant_pool: &Vec<CpInfo>) -> (String, Self) {
    let access_flags = fi.access_flags;

    let attributes = fi
      .attributes
      .iter()
      .map(|val| MiniAttribute::from(val, &constant_pool))
      .collect();

    let CpInfo::Utf8 { tag:_, length:_, bytes: name } = &constant_pool[fi.name_index as usize - 1] else { panic!() };
    let CpInfo::Utf8 { tag:_, length:_, bytes: descriptor } = &constant_pool[fi.descriptor_index as usize - 1] else { panic!() };
    (name.to_string() + descriptor, Self {
      access_flags,
      attributes
    })
  }
}

#[derive(Debug)]
pub struct MinifiedClassFile {
  access_flags: u16,
  this_class: MiniClassInfo,
  super_class: MiniClassInfo,
  interfaces: Vec<MiniClassInfo>,
  fields: HashMap<String, MiniFieldInfo>,
  methods: HashMap<String, MiniMethodInfo>,
  attributes: Vec<MiniAttribute>
}

impl MinifiedClassFile {
  pub fn from(cf: ClassFile) -> Self {
    let constant_pool = &cf.constant_pool;

    let access_flags = cf.access_flags;
    let CpInfo::Class { tag:_, name_index: index } = constant_pool[cf.this_class as usize - 1] else { panic!() };
    let CpInfo::Utf8 { tag:_, length:_, bytes: name } = &constant_pool[index as usize - 1] else { panic!() };
    let this_class = MiniClassInfo { name: name.to_string() };

    let CpInfo::Class { tag:_, name_index: index } = constant_pool[cf.super_class as usize - 1] else { panic!() };
    let CpInfo::Utf8 { tag:_, length:_, bytes: name } = &constant_pool[index as usize - 1] else { panic!() };
    let super_class = MiniClassInfo { name: name.to_string() };

    let interfaces: Vec<MiniClassInfo> = cf.interfaces.iter()
    .map(|class_index| {
      let CpInfo::Class { tag:_, name_index: index } = &constant_pool[*class_index as usize - 1] else { panic!() };
      let CpInfo::Utf8 { tag:_, length:_, bytes: name } = &constant_pool[*index as usize - 1] else { panic!() };
      MiniClassInfo{ name: name.to_string() }
    }).collect();

    let fields: HashMap<String, MiniFieldInfo> = cf
      .fields
      .iter()
      .map(|val| MiniFieldInfo::from(&val, constant_pool))
      .collect();
    
    let methods: HashMap<String, MiniMethodInfo> = cf
      .fields
      .iter()
      .map(|val| MiniMethodInfo::from(&val, constant_pool))
      .collect();

    let attributes = cf.attributes.iter().map(|val| MiniAttribute::from(val, &constant_pool)).collect();

    Self {
      access_flags,
      this_class,
      super_class,
      interfaces,
      fields,
      methods,
      attributes
    }
  }
}
