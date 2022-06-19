use self::{
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
};
use super::code::code_generator::Instructions;

pub mod annotation;
pub mod bootstrap_method;
pub mod classes;
pub mod element_value;
pub mod exception;
pub mod line_number;
pub mod local_variable;
pub mod local_variable_type;
pub mod parameter_annotation;
pub mod stack_map_frame;

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
