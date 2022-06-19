use self::attribute::ATTRIBUTE;
use super::cp_info::CpInfo;
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

#[derive(Debug, Clone)]
pub struct AttributeInfo {
  pub attribute_name_index: u16,
  pub attribute_length: u32,
  pub attribute: ATTRIBUTE
}

impl AttributeInfo {
  pub fn read(sr: &mut StreamReader, constant_pool: &Vec<CpInfo>) -> Self {
    let attribute_name_index = sr.get_u16();
    let attribute_length = sr.get_u32();
    let attribute = match &constant_pool[attribute_name_index as usize - 1] {
      CpInfo::Utf8 {
        tag: _,
        length: _,
        bytes
      } => match bytes.as_str() {
        "ConstantValue" => constant_value::read(sr),
        "Code" => code::read(sr, constant_pool),
        "StackMapTable" => stack_map_table::read(sr),
        "Exceptions" => exceptions::read(sr),
        "InnerClasses" => inner_classes::read(sr),
        "EnclosingMethod" => enclosing_method::read(sr),
        "Synthetic" => ATTRIBUTE::Synthetic,
        "Signature" => signature::read(sr),
        "SourceFile" => source_file::read(sr),
        "SourceDebugExtension" => source_debug_extensions::read(sr, attribute_length),
        "LineNumberTable" => line_number_table::read(sr),
        "LocalVariableTable" => local_variable_table::read(sr),
        "LocalVariableTypeTable" => local_variable_type_table::read(sr),
        "Deprecated" => ATTRIBUTE::Deprecated,
        "RuntimeVisibleAnnotations" => runtime_annotations::read::<true>(sr),
        "RuntimeInvisibleAnnotations" => runtime_annotations::read::<false>(sr),
        "RuntimeVisibleParameterAnnotations" => runtime_parameter_annotations::read::<true>(sr),
        "RuntimeInvisibleParameterAnnotations" => runtime_parameter_annotations::read::<false>(sr),
        "AnnotationDefault" => annotation_default::read(sr),
        "BootstrapMethods" => bootstrap_methods::read(sr),
        _ => todo!()
      },
      _ => panic!(
        "Constant at index {} was not a valid Utf8 identifier",
        attribute_name_index
      )
    };
    Self {
      attribute_name_index,
      attribute_length,
      attribute
    }
  }
}
