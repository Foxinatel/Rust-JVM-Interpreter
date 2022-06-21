use self::{
  attribute::{
    annotation::Annotation,
    bootstrap_method::BootstrapMethod,
    classes::Classes,
    element_value::ElementValue,
    exception::Exception,
    line_number::LineNumber,
    local_variable::LocalVariable,
    local_variable_type::LocalVariableType,
    parameter_annotation::ParameterAnnotation,
    stack_map_frame::StackMapFrame
  },
  code::code_generator::Instructions
};
use super::cp_info_resolved::ResolvedCpInfo;
use crate::stream_reader::StreamReader;

pub mod annotation_default;
pub mod attribute;
pub mod bootstrap_methods;
pub mod code;
pub mod constant_value;
pub mod enclosing_method;
pub mod exceptions;
pub mod inner_classes;
pub mod line_number_table;
pub mod local_variable_table;
pub mod local_variable_type_table;
pub mod runtime_annotations;
pub mod runtime_parameter_annotations;
pub mod signature;
pub mod source_debug_extensions;
pub mod source_file;
pub mod stack_map_table;

#[derive(Debug)]
pub enum Attribute {
  ConstantValue {
    // TODO!
    constantvalue_index: u16
  },
  Code {
    max_stack: u16,
    max_locals: u16,
    code: Vec<Instructions>,
    exception_table: Vec<Exception>,
    attributes: Vec<Attribute>
  },
  StackMapTable {
    entries: Vec<StackMapFrame>
  },
  Exceptions {
    exception_index_table: Vec<u16>
  },
  InnerClasses {
    classes: Vec<Classes>
  },
  EnclosingMethod {
    class_index: u16,
    method_index: u16
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
    attribute_name: String,
    attribute_length: u32,
    default_value: ElementValue
  },
  BootstrapMethods {
    bootstrap_methods: Vec<BootstrapMethod>
  }
}

impl Attribute {
  pub fn read(sr: &mut StreamReader, constant_pool: &Vec<ResolvedCpInfo>) -> Attribute {
    let attribute_name_index = sr.get_u16();
    let attribute_length = sr.get_u32();
    match &constant_pool[attribute_name_index as usize - 1] {
      ResolvedCpInfo::Utf8(string) => match string.as_str() {
        "ConstantValue" => constant_value::read(sr),
        "Code" => code::read(sr, constant_pool),
        "StackMapTable" => stack_map_table::read(sr),
        "Exceptions" => exceptions::read(sr),
        "InnerClasses" => inner_classes::read(sr),
        "EnclosingMethod" => enclosing_method::read(sr),
        "Synthetic" => Attribute::Synthetic,
        "Signature" => signature::read(sr, constant_pool),
        "SourceFile" => source_file::read(sr, constant_pool),
        "SourceDebugExtension" => source_debug_extensions::read(sr, attribute_length),
        "LineNumberTable" => line_number_table::read(sr),
        "LocalVariableTable" => local_variable_table::read(sr),
        "LocalVariableTypeTable" => local_variable_type_table::read(sr),
        "Deprecated" => Attribute::Deprecated,
        "RuntimeVisibleAnnotations" => runtime_annotations::read::<true>(sr),
        "RuntimeInvisibleAnnotations" => runtime_annotations::read::<false>(sr),
        "RuntimeVisibleParameterAnnotations" => runtime_parameter_annotations::read::<true>(sr),
        "RuntimeInvisibleParameterAnnotations" => runtime_parameter_annotations::read::<false>(sr),
        "AnnotationDefault" => annotation_default::read(sr, constant_pool),
        "BootstrapMethods" => bootstrap_methods::read(sr),
        _ => todo!()
      },
      _ => panic!("Constant at index {} was not a valid Utf8 identifier", attribute_name_index)
    }
  }
}
