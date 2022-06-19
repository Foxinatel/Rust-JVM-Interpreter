use super::{mini_class_info::MiniClassInfo, mini_name_and_type_info::NameAndTypeInfo};
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
  cp_info::CpInfo
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
    attributes: Vec<MiniAttribute>
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
        ATTRIBUTE::ConstantValue { constantvalue_index: _ } => todo!(),
        ATTRIBUTE::Code { max_stack, max_locals, code_length: _, code, exception_table_length: _, exception_table, attributes_count: _, attributes } => {
          MiniAttribute::Code { max_stack: *max_stack, max_locals: *max_locals, code: code.to_vec(), exception_table: exception_table.to_vec(), attributes: attributes
            .iter()
            .map(|val| MiniAttribute::from(val, &constant_pool))
            .collect() }
        },
        ATTRIBUTE::StackMapTable { number_of_entries: _, entries } => {
          MiniAttribute::StackMapTable { entries: entries.to_vec() }
        },
        ATTRIBUTE::Exceptions { number_of_exceptions: _, exception_index_table } => {
          MiniAttribute::Exceptions { exception_table: exception_index_table.iter().map(|class_index| {
            let CpInfo::Class { tag:_, name_index: index } = constant_pool[*class_index as usize - 1] else { panic!() };
            let CpInfo::Utf8 { tag:_, length:_, bytes: name } = &constant_pool[index as usize - 1] else { panic!() };
            MiniClassInfo{ name: name.to_string() }
          }).collect()}
        },
        ATTRIBUTE::InnerClasses { number_of_classes: _, classes } => {
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
        ATTRIBUTE::LineNumberTable { line_number_table_length: _, line_number_table } => {
          MiniAttribute::LineNumberTable { line_number_table: line_number_table.to_vec() }
        },
        ATTRIBUTE::LocalVariableTable { local_variable_table_length: _, local_variable_table } => {
          MiniAttribute::LocalVariableTable { local_variable_table: local_variable_table.to_vec() }
        },
        ATTRIBUTE::LocalVariableTypeTable { local_variable_type_table_length: _, local_variable_type_table } => {
          MiniAttribute::LocalVariableTypeTable { local_variable_type_table: local_variable_type_table.to_vec() }
        },
        ATTRIBUTE::Deprecated => MiniAttribute::Deprecated,
        ATTRIBUTE::RuntimeVisibleAnnotations { num_annotations: _, annotations } => {
          MiniAttribute::RuntimeVisibleAnnotations { annotations: annotations.to_vec() }
        },
        ATTRIBUTE::RuntimeInvisibleAnnotations { num_annotations: _, annotations } => {
          MiniAttribute::RuntimeInvisibleAnnotations { annotations: annotations.to_vec() }
        },
        ATTRIBUTE::RuntimeVisibleParameterAnnotations { num_annotations: _, parameter_annotations } => {
          MiniAttribute::RuntimeVisibleParameterAnnotations { parameter_annotations: parameter_annotations.to_vec() }
        },
        ATTRIBUTE::RuntimeInvisibleParameterAnnotations { num_annotations: _, parameter_annotations } => {
          MiniAttribute::RuntimeInvisibleParameterAnnotations { parameter_annotations: parameter_annotations.to_vec() }
        },
        ATTRIBUTE::AnnotationDefault { attribute_name_index: _, attribute_length: _, default_value: _ } => todo!(),
        ATTRIBUTE::BootstrapMethods { num_bootstrap_methods: _, bootstrap_methods } => {
          MiniAttribute::BootstrapMethods { bootstrap_methods: bootstrap_methods.to_vec() }
        }
    }
  }
}
