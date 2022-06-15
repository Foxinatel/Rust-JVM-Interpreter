use self::{
  stack_map_frame::StackMapFrame,
  exception::Exception,
  classes::Classes,
  line_number::LineNumber,
  local_variable::LocalVariable,
  annotation::Annotation,
  parameter_annotation::ParameterAnnotation,
  bootstrap_method::BootstrapMethod,
  local_variable_type::LocalVariableType,
  element_value::ElementValue
};

use super::AttributeInfo;

pub mod exception;
pub mod stack_map_frame;
pub mod classes;
pub mod line_number;
pub mod local_variable;
pub mod local_variable_type;
pub mod annotation;
pub mod parameter_annotation;
pub mod bootstrap_method;
pub mod element_value;

#[derive(Debug)]
pub enum ATTRIBUTE {
  ConstantValue {constantvalue_index: u16},
  Code {max_stack: u16, max_locals: u16, code_length: u32, code: Vec<u8>, exception_table_length: u16, exception_table: Vec<Exception>, attributes_count: u16, attributes: Vec<AttributeInfo>},
  StackMapTable {number_of_entries: u16, entries: Vec<StackMapFrame>},
  Exceptions {number_of_exceptions: u16, exception_index_table: Vec<u16>},
  InnerClasses {number_of_classes: u16, classes: Vec<Classes>},
  EnclosingMethod {class_index: u16, method_index: u16},
  Synthetic,
  Signature {signature_index: u16},
  SourceFile {sourcefile_index: u16},
  SourceDebugExtension {debug_extension: Vec<u8>},
  LineNumberTable {line_number_table_length: u16, line_number_table: Vec<LineNumber>},
  LocalVariableTable {local_variable_table_length: u16, local_variable_table: Vec<LocalVariable>},
  LocalVariableTypeTable {local_variable_type_table_length: u16, local_variable_type_table: Vec<LocalVariableType>},
  Deprecated,
  RuntimeVisibleAnnotations {num_annotations: u16, annotations: Vec<Annotation>},
  RuntimeInvisibleAnnotations {num_annotations: u16, annotations: Vec<Annotation>},
  RuntimeVisibleParameterAnnotations {num_annotations: u16, parameter_annotations: Vec<ParameterAnnotation>},
  RuntimeInvisibleParameterAnnotations {num_annotations: u16, parameter_annotations: Vec<ParameterAnnotation>},
  AnnotationDefault {attribute_name_index: u16, attribute_length: u32, default_value: ElementValue},
  BootstrapMethods {num_bootstrap_methods: u16, bootstrap_methods: Vec<BootstrapMethod>},

}